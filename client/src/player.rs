use bevy::{prelude::*, time::FixedTimestep, sprite::Anchor};

use crate::{
    components::{Player, Position},
    TIME_STEP,
};
pub struct PlayerPlugin;

pub const PLAYERSPEED: f32 = 100.;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_player).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_player),
        );
    }
}

fn init_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _loaded = asset_server.load_folder("game/characters/archer");
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.get_handle("game/characters/archer/wee_mons_archer_idle_d_1.png"),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 10.0,
                },
                scale : Vec3 { x: 2., y: 2., z: 2. },
                ..Default::default()
            },
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    let mut transform = query.get_single_mut().expect("Could not find player in move_player");
    let mut move_input: Vec2 = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.just_released(KeyCode::Up) {
        move_input.y = 1.;
    }
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.just_released(KeyCode::Down) {
        move_input.y = -1.;
    }
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.just_released(KeyCode::Left) {
        move_input.x = -1.;
    }
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.just_released(KeyCode::Right) {
        move_input.x = 1.;
    }

    // Arena bounds
    // x: -210, 210
    // y : -160, 120,
    let mut new_player_position = transform.translation + move_input.extend(0.) * PLAYERSPEED * time.delta_seconds();
    new_player_position.x = new_player_position.x.clamp(-220., 185.);
    new_player_position.y = new_player_position.y.clamp(-180., 100.);
    transform.translation = new_player_position;
}
