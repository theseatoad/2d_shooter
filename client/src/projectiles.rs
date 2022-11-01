use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    components::{Collider, EMovementDirection, Projectile, Velocity},
    GameState, TIME_STEP,
};

/**
 * CONSTANTS
 */

// If projectile mask is player, it will not effect player. (Visa-versa for enemy)
pub enum ProjectileMask {
    Player,
    Enemy,
}
#[derive(Bundle)]
pub struct ArcherArrow {
    #[bundle]
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    collider: Collider,
    projectile: Projectile,
}

impl ArcherArrow {
    /**
     * Let
     */
    pub fn new(
        speed: f32,
        location: &Vec3,
        arrow_direction: &EMovementDirection,
        arrow_handle_images: &Vec<Handle<Image>>,
    ) -> ArcherArrow {
        let a_velocity: Vec2;
        let arrow_texture: Handle<Image>;
        let flip_x: bool;
        match arrow_direction {
            EMovementDirection::UP => {
                a_velocity = Vec2 { x: 0.0, y: 1.0 } * speed;
                arrow_texture = arrow_handle_images.get(2).unwrap().clone();
                flip_x = false;
            }
            EMovementDirection::UPRIGHT => {
                a_velocity = Vec2 { x: 0.66, y: 0.66 } * speed;
                arrow_texture = arrow_handle_images.get(1).unwrap().clone();
                flip_x = true;
            }
            EMovementDirection::UPLEFT => {
                a_velocity = Vec2 { x: -0.66, y: 0.66 } * speed;
                arrow_texture = arrow_handle_images.get(1).unwrap().clone();
                flip_x = false;
            }
            EMovementDirection::RIGHT => {
                a_velocity = Vec2 { x: 1.0, y: 0.0 } * speed;
                arrow_texture = arrow_handle_images.get(0).unwrap().clone();
                flip_x = false;
            }
            EMovementDirection::DOWN => {
                a_velocity = Vec2 { x: 0.0, y: -1.0 } * speed;
                arrow_texture = arrow_handle_images.get(2).unwrap().clone();
                flip_x = false;
            }
            EMovementDirection::DOWNRIGHT => {
                a_velocity = Vec2 { x: 0.6, y: -0.6 } * speed;
                arrow_texture = arrow_handle_images.get(1).unwrap().clone();
                flip_x = false;
            }
            EMovementDirection::DOWNLEFT => {
                a_velocity = Vec2 { x: -0.6, y: -0.6 } * speed;
                arrow_texture = arrow_handle_images.get(1).unwrap().clone();
                flip_x = true;
            }
            EMovementDirection::LEFT => {
                a_velocity = Vec2 { x: -1.0, y: 0.0 } * speed;
                arrow_texture = arrow_handle_images.get(0).unwrap().clone();
                flip_x = false;
            }
        };
        ArcherArrow {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: location.x,
                        y: location.y,
                        z: location.z + 1.0,
                    },
                    scale: Vec3 {
                        x: 2.0,
                        y: 2.0,
                        z: 2.0,
                    },
                    ..default()
                },
                sprite: Sprite {
                    flip_x,
                    ..default()
                },
                texture: arrow_texture,
                ..default()
            },
            collider: Collider,
            velocity: Velocity(a_velocity),
            projectile: Projectile(ProjectileMask::Player),
        }
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_projectiles)
                .with_system(check_projectile_collisions),
        );
    }
}

fn move_projectiles(
    mut query: Query<(&mut Transform, &Velocity), With<Projectile>>,
    time: Res<Time>,
) {
    for (mut projectile_transform, projectile_velocity) in &mut query {
        projectile_transform.translation += projectile_velocity.0.extend(0.) * time.delta_seconds();
    }
}

fn check_projectile_collisions(
    mut query: Query<(Entity, &Transform), With<Projectile>>,
    mut commands: Commands,
) {
    // Kill projectiles that are on or over border of map.
    for (entity, projectile_transform) in &mut query {
        // Arena bounds
        // x: -210, 210
        // y : -160, 120,
        if projectile_transform.translation.x >= 210.0
            || projectile_transform.translation.x <= -210.0
        {
            commands.entity(entity).despawn_recursive();
        } else if projectile_transform.translation.y >= 120.0
            || projectile_transform.translation.y <= -160.0
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}
