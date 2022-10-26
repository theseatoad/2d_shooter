use bevy::prelude::*;

use crate::components::{Collider, EMovementDirection, Velocity};

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
                a_velocity = Vec2 { x: 0.0 , y: 1.0 } * speed;
                arrow_texture = arrow_handle_images.get(2).unwrap().clone();
                flip_x = false;
            }
            EMovementDirection::UPRIGHT => {
                a_velocity = Vec2 { x: 0.66 , y: 0.66 } * speed;
                arrow_texture = arrow_handle_images.get(1).unwrap().clone();
                flip_x = true;
            }
            EMovementDirection::UPLEFT => {
               a_velocity = Vec2 { x: -0.66 , y: 0.66 } * speed;
               arrow_texture = arrow_handle_images.get(1).unwrap().clone();
               flip_x = false;
            }
            EMovementDirection::RIGHT => {
               a_velocity = Vec2 { x: 1.0 , y: 0.0 } * speed;
               arrow_texture = arrow_handle_images.get(0).unwrap().clone();
               flip_x = false;
            }
            EMovementDirection::DOWN => {
               a_velocity = Vec2 { x: 0.0 , y: -1.0 } * speed;
               arrow_texture = arrow_handle_images.get(2).unwrap().clone();
               flip_x = false;
            }
            EMovementDirection::DOWNRIGHT => {
               a_velocity = Vec2 { x: 0.6 , y: -0.6 } * speed;
               arrow_texture = arrow_handle_images.get(1).unwrap().clone();
               flip_x = false;
            }
            EMovementDirection::DOWNLEFT => {
               a_velocity = Vec2 { x: -0.6 , y: -0.6 } * speed;
               arrow_texture = arrow_handle_images.get(1).unwrap().clone();
               flip_x = true;
            }
            EMovementDirection::LEFT => {
               a_velocity = Vec2 { x: -1.0 , y: 0.0 } * speed;
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
        }
    }
}
