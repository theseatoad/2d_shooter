use std::time::Duration;

use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    components::{AnimationTimer, CharacterState, Direction, ECharacterState, EDirection, Player},
    TIME_STEP,
};
pub struct PlayerPlugin;

pub const PLAYERSPEED: f32 = 100.;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_player).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_player)
                .with_system(animate_sprite),
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
        .insert(Direction(EDirection::default()))
        .insert(CharacterState(ECharacterState::default()))
        .insert(AnimationTimer(Timer::from_seconds(0.25, true)));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut Transform,
            &mut Direction,
            &mut AnimationTimer,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    for (mut transform, mut direction, mut timer) in &mut query {
        let mut move_input: Vec2 = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.just_released(KeyCode::Up) {
            move_input.y = 1.;
            if direction.0 != EDirection::UP {
                direction.0 = EDirection::UP;
                timer.0.set_elapsed(Duration::from_secs(20));
            }
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.just_released(KeyCode::Down) {
            move_input.y = -1.;
            if direction.0 != EDirection::DOWN {
                direction.0 = EDirection::DOWN;
                timer.0.set_elapsed(Duration::from_secs(20));
            }
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.just_released(KeyCode::Left) {
            move_input.x = -1.;
            if direction.0 != EDirection::LEFT {
                direction.0 = EDirection::LEFT;
                timer.0.set_elapsed(Duration::from_secs(20));
            }
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.just_released(KeyCode::Right) {
            move_input.x = 1.;
            if direction.0 != EDirection::RIGHT {
                direction.0 = EDirection::RIGHT;
                timer.0.set_elapsed(Duration::from_secs(20));
            }
        }
        // Arena bounds
        // x: -210, 210
        // y : -160, 120,
        let mut new_player_position =
            transform.translation + move_input.extend(0.) * PLAYERSPEED * time.delta_seconds();
        new_player_position.x = new_player_position.x.clamp(-220., 185.);
        new_player_position.y = new_player_position.y.clamp(-180., 100.);
        transform.translation = new_player_position;
    };
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &crate::components::Direction,
            &CharacterState,
        ),
        With<Player>,
    >,
) {
    for (mut timer, mut sprite, direction, character_state) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            match direction.0 {
                EDirection::LEFT => {
                    if character_state.0 == ECharacterState::IDLE {
                        if sprite.index != 22 {
                            sprite.index = 22;
                        } else {
                            sprite.index = 24;
                        }
                    } else if character_state.0 == ECharacterState::ATTACK {
                        if sprite.index == 0 {
                            sprite.index = 2;
                        } else {
                            sprite.index = 0;
                        }
                    };
                }
                EDirection::RIGHT => {
                    if character_state.0 == ECharacterState::IDLE {
                        if sprite.index != 26 {
                            sprite.index = 26;
                        } else {
                            sprite.index = 28;
                        }
                    } else if character_state.0 == ECharacterState::ATTACK {
                        if sprite.index == 0 {
                            sprite.index = 2;
                        } else {
                            sprite.index = 0;
                        }
                    };
                }
                EDirection::UP => {
                    if character_state.0 == ECharacterState::IDLE {
                        if sprite.index != 30 {
                            sprite.index = 30;
                        } else {
                            sprite.index = 32;
                        }
                    } else if character_state.0 == ECharacterState::ATTACK {
                        if sprite.index == 0 {
                            sprite.index = 2;
                        } else {
                            sprite.index = 0;
                        }
                    };
                }
                EDirection::DOWN => {
                    if character_state.0 == ECharacterState::IDLE {
                        if sprite.index != 18 {
                            sprite.index = 18;
                        } else {
                            sprite.index = 20;
                        }
                    } else if character_state.0 == ECharacterState::ATTACK {
                        if sprite.index == 0 {
                            sprite.index = 2;
                        } else {
                            sprite.index = 0;
                        }
                    };
                }
            };
        }
    }
}
