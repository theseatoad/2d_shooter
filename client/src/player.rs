use std::time::Duration;

use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    assets::GameAssets,
    components::{
        AnimationTimer, AttackDirection, AttackTimer, CharacterState, ECharacterState,
        EMovementDirection, ESpriteDirection, Player, SpriteDirection,
    },
    projectiles::ArcherArrow,
    GameState, TIME_STEP,
};
pub struct PlayerPlugin;

pub const PLAYERSPEED: f32 = 100.;
// IN SECONDS
pub const ATTACK_ANIM_SPEED: f32 = 0.10;
pub const IDLE_ANIM_SPEED: f32 = 0.25;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(init_player))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(player_move)
                    .with_system(animate_sprite)
                    .with_system(player_attack.after(player_move)),
            );
    }
}

fn init_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_assets.archer_tileset.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                scale: Vec3 {
                    x: 2.0,
                    y: 2.0,
                    z: 2.0,
                },
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(SpriteDirection(ESpriteDirection::default()))
        .insert(AttackDirection(EMovementDirection::default()))
        .insert(CharacterState(ECharacterState::default()))
        .insert(AnimationTimer(Timer::from_seconds(IDLE_ANIM_SPEED, true)))
        .insert(AttackTimer(Timer::from_seconds(ATTACK_ANIM_SPEED, true)));
}

fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut SpriteDirection, &mut AttackDirection), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut sprite_direction, mut attack_direction) in &mut query {
        let mut move_input: Vec2 = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            move_input.y = 1.;
            sprite_direction.0 = ESpriteDirection::UP;
        }
        if keyboard_input.pressed(KeyCode::S) {
            move_input.y = -1.;
            sprite_direction.0 = ESpriteDirection::DOWN;
        }
        if keyboard_input.pressed(KeyCode::A) {
            move_input.x = -1.;
            sprite_direction.0 = ESpriteDirection::LEFT;
        }
        if keyboard_input.pressed(KeyCode::D) {
            move_input.x = 1.;
            sprite_direction.0 = ESpriteDirection::RIGHT;
        }

        // "Default" for attack direction since it is dependent on the sprite direction for the non-movement case.
        if move_input.x == 0. && move_input.y == 0. {
            match sprite_direction.0 {
                ESpriteDirection::UP => attack_direction.0 = EMovementDirection::UP,
                ESpriteDirection::RIGHT => attack_direction.0 = EMovementDirection::RIGHT,
                ESpriteDirection::DOWN => attack_direction.0 = EMovementDirection::DOWN,
                ESpriteDirection::LEFT => attack_direction.0 = EMovementDirection::LEFT,
            }
            // These will be clockwise (to make sure I cover all cases. Would be better to have a type so we can ensure through match it is exhaustive.)
        } else if move_input.x == 0. && move_input.y == 1. {
            attack_direction.0 = EMovementDirection::UP;
        } else if move_input.x == 1. && move_input.y == 1. {
            attack_direction.0 = EMovementDirection::UPRIGHT;
        } else if move_input.x == 1. && move_input.y == 0. {
            attack_direction.0 = EMovementDirection::RIGHT;
        } else if move_input.x == 1. && move_input.y == -1. {
            attack_direction.0 = EMovementDirection::DOWNRIGHT;
        } else if move_input.x == 0. && move_input.y == -1. {
            attack_direction.0 = EMovementDirection::DOWN;
        } else if move_input.x == -1. && move_input.y == -1. {
            attack_direction.0 = EMovementDirection::DOWNLEFT;
        } else if move_input.x == -1. && move_input.y == 0. {
            attack_direction.0 = EMovementDirection::LEFT;
        } else if move_input.x == -1. && move_input.y == 1. {
            attack_direction.0 = EMovementDirection::UPLEFT;
        }
        // Arena bounds
        // x: -210, 210
        // y : -160, 120,
        let mut new_player_position =
            transform.translation + move_input.extend(0.) * PLAYERSPEED * time.delta_seconds();
        new_player_position.x = new_player_position.x.clamp(-220., 185.);
        new_player_position.y = new_player_position.y.clamp(-180., 100.);
        transform.translation = new_player_position;
    }
}

fn player_attack(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &Transform,
            &AttackDirection,
            &mut AttackTimer,
            &mut CharacterState,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
) {
    for (transform, attack_direction, mut timer, mut character_state) in &mut query {
        if character_state.0 == ECharacterState::ATTACK {
            timer.tick(time.delta());
        }
        if timer.just_finished() {
            //Makes sure to update the attack state (Useful for syncing animations and if player can attack again)
            if character_state.0 == ECharacterState::ATTACK {
                commands.spawn_bundle(ArcherArrow::new(
                    1.0,
                    &transform.translation,
                    &attack_direction.0,
                    &game_assets.archer_arrows,
                ));
                character_state.0 = ECharacterState::IDLE;
            }
        }
        //Means it can attack
        if character_state.0 == ECharacterState::IDLE && keyboard_input.pressed(KeyCode::Space) {
            character_state.0 = ECharacterState::ATTACK;
            timer.reset();
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &crate::components::SpriteDirection,
            &CharacterState,
        ),
        With<Player>,
    >,
) {
    for (mut timer, mut sprite, sprite_direction, character_state) in &mut query {
        let mut next_sprite = false;
        timer.tick(time.delta());
        if timer.just_finished() {
            next_sprite = true;
        }
        match sprite_direction.0 {
            ESpriteDirection::LEFT => {
                if character_state.0 == ECharacterState::IDLE {
                    //Sets to first sprite instantly.
                    if sprite.index != 11 && sprite.index != 12 {
                        sprite.index = 11;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(IDLE_ANIM_SPEED));
                    }
                    // Sets (typically second) to first.
                    else if sprite.index != 11 && next_sprite {
                        sprite.index = 11;
                    // Sets the first to second.
                    } else if sprite.index == 11 && next_sprite {
                        sprite.index = 12;
                    }
                } else if character_state.0 == ECharacterState::ATTACK {
                    //Sets to first sprite instantly.
                    if sprite.index != 2 && sprite.index != 3 {
                        sprite.index = 2;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(ATTACK_ANIM_SPEED));
                    }
                    // Sets (typically second) to first.
                    else if sprite.index != 2 && next_sprite {
                        sprite.index = 2;
                    }
                    // Sets the first to second.
                    else if sprite.index == 2 && next_sprite {
                        sprite.index = 3;
                    }
                };
            }
            ESpriteDirection::RIGHT => {
                if character_state.0 == ECharacterState::IDLE {
                    if sprite.index != 13 && sprite.index != 14 {
                        sprite.index = 13;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(IDLE_ANIM_SPEED));
                    } else if sprite.index != 13 && next_sprite {
                        sprite.index = 13;
                    } else if sprite.index == 13 && next_sprite {
                        sprite.index = 14;
                    }
                } else if character_state.0 == ECharacterState::ATTACK {
                    //Sets to first sprite instantly.
                    if sprite.index != 4 && sprite.index != 5 {
                        sprite.index = 4;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(ATTACK_ANIM_SPEED));
                    }
                    // Sets (typically second) to first.
                    else if sprite.index != 4 && next_sprite {
                        sprite.index = 4;
                    }
                    // Sets the first to second.
                    else if sprite.index == 4 && next_sprite {
                        sprite.index = 5;
                    }
                };
            }
            ESpriteDirection::UP => {
                if character_state.0 == ECharacterState::IDLE {
                    if sprite.index != 15 && sprite.index != 16 {
                        sprite.index = 15;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(IDLE_ANIM_SPEED));
                    } else if sprite.index != 15 && next_sprite {
                        sprite.index = 15;
                    } else if sprite.index == 15 && next_sprite {
                        sprite.index = 16;
                    }
                } else if character_state.0 == ECharacterState::ATTACK {
                    //Sets to first sprite instantly.
                    if sprite.index != 6 && sprite.index != 7 {
                        sprite.index = 6;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(ATTACK_ANIM_SPEED));
                    }
                    // Sets (typically second) to first.
                    else if sprite.index != 6 && next_sprite {
                        sprite.index = 6;
                    }
                    // Sets the first to second.
                    else if sprite.index == 6 && next_sprite {
                        sprite.index = 7;
                    }
                };
            }
            ESpriteDirection::DOWN => {
                if character_state.0 == ECharacterState::IDLE {
                    if sprite.index != 9 && sprite.index != 10 {
                        sprite.index = 9;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(IDLE_ANIM_SPEED));
                    } else if sprite.index != 9 && next_sprite {
                        sprite.index = 9;
                    } else if sprite.index == 9 && next_sprite {
                        sprite.index = 10;
                    }
                } else if character_state.0 == ECharacterState::ATTACK {
                    //Sets to first sprite instantly.
                    if sprite.index != 0 && sprite.index != 1 {
                        sprite.index = 0;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(ATTACK_ANIM_SPEED));
                    }
                    // Sets (typically second) to first.
                    else if sprite.index != 0 && next_sprite {
                        sprite.index = 0;
                    }
                    // Sets the first to second.
                    else if sprite.index == 0 && next_sprite {
                        sprite.index = 1;
                    }
                };
            }
        };
    }
}
