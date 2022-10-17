use std::time::Duration;

use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    components::{
        AnimationTimer, AttackDirection, AttackTimer, CharacterState, ECharacterState,
        EMovementDirection, ESpriteDirection, Player, SpriteDirection,
    },
    GameState, TIME_STEP,
};
pub struct PlayerPlugin;

pub const PLAYERSPEED: f32 = 100.;
// IN SECONDS
pub const ATTACK_ANIM_SPEED : f32 = 0.10;
pub const IDLE_ANIM_SPEED : f32 = 0.25;

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

fn init_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let _loaded = asset_server.load_folder("game/characters/archer");
    //Note that every other is empty
    let texture_handle = asset_server.get_handle("game/characters/archer/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(10.0, 10.0),
        33,
        1,
        Vec2::new(0.0, 0.),
        Vec2::new(5.0, 5.0),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
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
) {
    for (transform, attack_direciton, mut timer, mut character_state) in &mut query {
        println!("Character State : {:?}", character_state.0);
        if character_state.0 == ECharacterState::ATTACK {
            timer.tick(time.delta());
            println!("{:?}", timer.elapsed());
        }
        if timer.just_finished() {
            //Makes sure to update the attack state (Useful for syncing animations and if player can attack again)
            if character_state.0 == ECharacterState::ATTACK {
                character_state.0 = ECharacterState::IDLE;
            }
        }
        //Means it can attack
        if character_state.0 == ECharacterState::IDLE && keyboard_input.pressed(KeyCode::Space) {
            character_state.0 = ECharacterState::ATTACK;
            timer.reset();
        }
        if character_state.0 == ECharacterState::ATTACK {
            println!("ATTACKING")
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
                    if sprite.index != 22 && sprite.index != 24 {
                        sprite.index = 22;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(IDLE_ANIM_SPEED));
                    }
                    // Sets (typically second) to first.
                    else if sprite.index != 22 && next_sprite {
                        sprite.index = 22;
                    // Sets the first to second.
                    } else if sprite.index == 22 && next_sprite {
                        sprite.index = 24;
                    }
                } else if character_state.0 == ECharacterState::ATTACK {
                    //Sets to first sprite instantly.
                    if sprite.index != 4 && sprite.index != 6 {
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
                        sprite.index = 6;
                    }
                };
            }
            ESpriteDirection::RIGHT => {
                if character_state.0 == ECharacterState::IDLE {
                    if sprite.index != 26 && sprite.index != 28 {
                        sprite.index = 26;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(IDLE_ANIM_SPEED));
                    } else if sprite.index != 26 && next_sprite {
                        sprite.index = 26;
                    } else if sprite.index == 26 && next_sprite {
                        sprite.index = 28;
                    }
                } else if character_state.0 == ECharacterState::ATTACK {
                    //Sets to first sprite instantly.
                    if sprite.index != 8 && sprite.index != 10 {
                        sprite.index = 8;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(ATTACK_ANIM_SPEED));
                    }
                    // Sets (typically second) to first.
                    else if sprite.index != 8 && next_sprite {
                        sprite.index = 8;
                    }
                    // Sets the first to second.
                    else if sprite.index == 8 && next_sprite {
                        sprite.index = 10;
                    }
                };
            }
            ESpriteDirection::UP => {
                if character_state.0 == ECharacterState::IDLE {
                    if sprite.index != 30 && sprite.index != 32 {
                        sprite.index = 30;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(IDLE_ANIM_SPEED));
                    } else if sprite.index != 30 && next_sprite {
                        sprite.index = 30;
                    } else if sprite.index == 30 && next_sprite {
                        sprite.index = 32;
                    }
                } else if character_state.0 == ECharacterState::ATTACK {
                    //Sets to first sprite instantly.
                    if sprite.index != 12 && sprite.index != 14 {
                        sprite.index = 12;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(ATTACK_ANIM_SPEED));
                    }
                    // Sets (typically second) to first.
                    else if sprite.index != 12 && next_sprite {
                        sprite.index = 12;
                    }
                    // Sets the first to second.
                    else if sprite.index == 12 && next_sprite {
                        sprite.index = 14;
                    }
                };
            }
            ESpriteDirection::DOWN => {
                if character_state.0 == ECharacterState::IDLE {
                    if sprite.index != 18 && sprite.index != 20 {
                        sprite.index = 18;
                        timer.reset();
                        timer.set_duration(Duration::from_secs_f32(IDLE_ANIM_SPEED));
                     } else if sprite.index != 18 && next_sprite {
                        sprite.index = 18;
                    } else if sprite.index == 18 && next_sprite {
                        sprite.index = 20;
                    }
                } else if character_state.0 == ECharacterState::ATTACK {
                    //Sets to first sprite instantly.
                    if sprite.index != 0 && sprite.index != 2 {
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
                        sprite.index = 2;
                    }
                };
            }
        };
    }
}
