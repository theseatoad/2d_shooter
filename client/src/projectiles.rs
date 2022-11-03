use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    components::{Collider, ECharacterAttackState, Projectile, Velocity},
    game::{MAP_DOWN_BOUND, MAP_LEFT_BOUND, MAP_RIGHT_BOUND, MAP_UP_BOUND},
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
        arrow_direction: &ECharacterAttackState,
        arrow_handle_images: &Vec<Handle<Image>>,
    ) -> ArcherArrow {
        let a_velocity: Vec2;
        let arrow_texture: Handle<Image>;
        let flip_x: bool;
        match arrow_direction {
            ECharacterAttackState::ATTACK_UP => {
                a_velocity = Vec2 { x: 0.0, y: 1.0 } * speed;
                arrow_texture = arrow_handle_images.get(2).unwrap().clone();
                flip_x = false;
            }
            ECharacterAttackState::ATTACK_UPRIGHT => {
                a_velocity = Vec2 { x: 0.66, y: 0.66 } * speed;
                arrow_texture = arrow_handle_images.get(1).unwrap().clone();
                flip_x = true;
            }
            ECharacterAttackState::ATTACK_UPLEFT => {
                a_velocity = Vec2 { x: -0.66, y: 0.66 } * speed;
                arrow_texture = arrow_handle_images.get(1).unwrap().clone();
                flip_x = false;
            }
            ECharacterAttackState::ATTACK_RIGHT => {
                a_velocity = Vec2 { x: 1.0, y: 0.0 } * speed;
                arrow_texture = arrow_handle_images.get(0).unwrap().clone();
                flip_x = false;
            }
            ECharacterAttackState::ATTACK_DOWN => {
                a_velocity = Vec2 { x: 0.0, y: -1.0 } * speed;
                arrow_texture = arrow_handle_images.get(2).unwrap().clone();
                flip_x = false;
            }
            ECharacterAttackState::ATTACK_DOWNRIGHT => {
                a_velocity = Vec2 { x: 0.6, y: -0.6 } * speed;
                arrow_texture = arrow_handle_images.get(1).unwrap().clone();
                flip_x = false;
            }
            ECharacterAttackState::ATTACK_DOWNLEFT => {
                a_velocity = Vec2 { x: -0.6, y: -0.6 } * speed;
                arrow_texture = arrow_handle_images.get(1).unwrap().clone();
                flip_x = true;
            }
            ECharacterAttackState::ATTACK_LEFT => {
                a_velocity = Vec2 { x: -1.0, y: 0.0 } * speed;
                arrow_texture = arrow_handle_images.get(0).unwrap().clone();
                flip_x = false;
            }
            ECharacterAttackState::IDLE => {
               /* It should never get here. */
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
                        z: location.z - 0.1,
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
        if projectile_transform.translation.x >= MAP_RIGHT_BOUND
            || projectile_transform.translation.x <= MAP_LEFT_BOUND
        {
            commands.entity(entity).despawn_recursive();
        } else if projectile_transform.translation.y >= MAP_UP_BOUND
            || projectile_transform.translation.y <= MAP_DOWN_BOUND
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}
