use bevy::prelude::*;

use crate::{
    animation::AnimationPlugin, assets::GameAssets, player::PlayerPlugin,
    projectiles::ProjectilePlugin, GameState,
};

pub const MAP_LEFT_BOUND: f32 = -210.0;
pub const MAP_UP_BOUND: f32 = 120.0;
pub const MAP_RIGHT_BOUND: f32 = 210.0;
pub const MAP_DOWN_BOUND: f32 = -160.0;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(ProjectilePlugin)
            .add_plugin(AnimationPlugin)
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_map));
    }
}

fn setup_map(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Spawn edge tiles
    commands.spawn_bundle(SpriteBundle {
        texture: game_assets.arena_background.clone(),
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            ..Default::default()
        },
        ..Default::default()
    });
}
