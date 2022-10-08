use bevy::{prelude::*, time::FixedTimestep};

use crate::{GameState, TIME_STEP};
pub struct GamePlugin;

#[derive(Component, Default, Clone)]
pub struct OnlyInGame;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_map))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
                .with_system(move_player.before(check_for_collisions))
                .with_system(apply_velocity.before(check_for_collisions)),
        );
    }
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn edge tiles
    commands
    .spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    })
    .insert(OnlyInGame)
    .with_children(|parent| {
        parent.spawn_bundle(ImageBundle {
            style: Style {
                align_self: AlignSelf::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            image: asset_server.load("arenascreen.png").into(),
            ..Default::default()
        });
    })
    .insert(OnlyInGame);
    // Spawn self-player
}

/**
 * Checks for collisions between player's and bullets
 */
fn check_for_collisions(){

}

/**
 * Moves player
 */
fn move_player(){

}

fn apply_velocity() {

}