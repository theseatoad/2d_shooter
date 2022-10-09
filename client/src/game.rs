use bevy::prelude::*;

use crate::{player::PlayerPlugin, GameState};
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_map));
    }
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn edge tiles
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("game/arenascreen.png"),
        transform : Transform {
            translation : Vec3 {
                x : 0.0,
                y : 0.0,
                z : 0.0,
            },
            ..Default::default()
        },
        ..Default::default()
    });
}
