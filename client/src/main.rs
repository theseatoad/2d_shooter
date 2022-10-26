use assets::{MainMenuAssets, GameAssets};
use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_asset_loader::prelude::*;
const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    AssetLoading,
    MainMenu,
    InGame,
}

impl Default for GameState {
    fn default() -> GameState {
        GameState::MainMenu
    }
}

mod components;
mod game;
mod mainmenu;
mod player;
mod ui;
mod projectiles;
mod assets;
fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "2d_shooter".to_string(),
            width: 480.,
            height: 384.,
            resizable: false,
            ..default()
        })
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::MainMenu)
                .with_collection::<MainMenuAssets>()
                .with_collection::<GameAssets>()
        )
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_system(bevy::window::close_on_esc)
        .add_plugin(mainmenu::MainMenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_state(GameState::AssetLoading)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
