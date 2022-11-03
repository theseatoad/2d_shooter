use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    assets::GameAssets,
    components::*,
    game::{MAP_DOWN_BOUND, MAP_LEFT_BOUND, MAP_RIGHT_BOUND, MAP_UP_BOUND},
    projectiles::ArcherArrow,
    GameState, TIME_STEP,
};
pub struct PlayerPlugin;
pub const ARCHER_PROJECTILE_SPEED: f32 = 500.;
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
                    .with_system(player_attack),
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
        .insert(CharacterState((
            ECharacterMovementState::default(),
            ECharacterAttackState::default(),
        )))
        .insert(AnimationTimer(Timer::from_seconds(IDLE_ANIM_SPEED, true)))
        .insert(AttackTimer(Timer::from_seconds(ATTACK_ANIM_SPEED, true)));
}

fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CharacterState), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut character_state) in &mut query {
        let mut move_input: Vec2 = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            move_input.y = 1.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            move_input.y = -1.;
        }
        if keyboard_input.pressed(KeyCode::A) {
            move_input.x = -1.;
        }
        if keyboard_input.pressed(KeyCode::D) {
            move_input.x = 1.;
        }
        if move_input.x == 0.0 && move_input.y == 0.0 {
            if character_state.0 .0 != ECharacterMovementState::IDLE {
                character_state.0 .0 = ECharacterMovementState::IDLE;
            }
        } else if move_input.x == 1.0 {
            if character_state.0 .0 != ECharacterMovementState::WALK_RIGHT {
                character_state.0 .0 = ECharacterMovementState::WALK_RIGHT;
            }
        } else if move_input.x == -1.0 {
            if character_state.0 .0 != ECharacterMovementState::WALK_LEFT {
                character_state.0 .0 = ECharacterMovementState::WALK_LEFT;
            }
        } else if move_input.y == 1.0 {
            if character_state.0 .0 != ECharacterMovementState::WALK_UP {
                character_state.0 .0 = ECharacterMovementState::WALK_UP;
            }
        } else if move_input.y == -1.0 {
            if character_state.0 .0 != ECharacterMovementState::WALK_DOWN {
                character_state.0 .0 = ECharacterMovementState::WALK_DOWN;
            }
        }
        let mut new_player_position =
            transform.translation + move_input.extend(0.) * PLAYERSPEED * time.delta_seconds();
        new_player_position.x = new_player_position.x.clamp(MAP_LEFT_BOUND, MAP_RIGHT_BOUND);
        new_player_position.y = new_player_position.y.clamp(MAP_DOWN_BOUND, MAP_UP_BOUND);
        transform.translation = new_player_position;
    }
}

fn player_attack(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Transform, &mut AttackTimer, &mut CharacterState), With<Player>>,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
    audio: Res<Audio>
) {
    for (transform, mut timer, mut character_state) in &mut query {
        if character_state.0 .1 != ECharacterAttackState::IDLE {
            timer.tick(time.delta());
        }
        if timer.just_finished() {
            //Makes sure to update the attack state
            character_state.0 .1 = ECharacterAttackState::IDLE;
            timer.reset();
        }
        //Means it can attack
        if character_state.0 .1 == ECharacterAttackState::IDLE {
            let mut should_attack = false;
            // Up Right
            if keyboard_input.pressed(KeyCode::Up) && keyboard_input.pressed(KeyCode::Right) {
                character_state.0 .1 = ECharacterAttackState::ATTACK_UPRIGHT;
                should_attack = true;
            }
            // Down Right
            else if keyboard_input.pressed(KeyCode::Down)
                && keyboard_input.pressed(KeyCode::Right)
            {
                character_state.0 .1 = ECharacterAttackState::ATTACK_DOWNRIGHT;
                should_attack = true;
            }
            // Down Left
            else if keyboard_input.pressed(KeyCode::Down) && keyboard_input.pressed(KeyCode::Left)
            {
                character_state.0 .1 = ECharacterAttackState::ATTACK_DOWNLEFT;
                should_attack = true;
            }
            // Up Left
            else if keyboard_input.pressed(KeyCode::Up) && keyboard_input.pressed(KeyCode::Left) {
                character_state.0 .1 = ECharacterAttackState::ATTACK_UPLEFT;
                should_attack = true;
            }
            // Up
            else if keyboard_input.pressed(KeyCode::Up) {
                character_state.0 .1 = ECharacterAttackState::ATTACK_UP;
                should_attack = true;
            }
            // Right
            else if keyboard_input.pressed(KeyCode::Right) {
                character_state.0 .1 = ECharacterAttackState::ATTACK_RIGHT;
                should_attack = true;
            }
            // Down
            else if keyboard_input.pressed(KeyCode::Down) {
                character_state.0 .1 = ECharacterAttackState::ATTACK_DOWN;
                should_attack = true;
            }
            // Left
            else if keyboard_input.pressed(KeyCode::Left) {
                character_state.0 .1 = ECharacterAttackState::ATTACK_LEFT;
                should_attack = true;
            }
            if should_attack {
                commands.spawn_bundle(ArcherArrow::new(
                    ARCHER_PROJECTILE_SPEED,
                    &transform.translation,
                    &character_state.0 .1,
                    &game_assets.archer_arrows,
                ));
                audio.play(game_assets.arrow_noise.clone());
            }
        }
    }
}
