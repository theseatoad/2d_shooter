use bevy::prelude::*;

use crate::projectiles::ProjectileMask;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct SpriteDirection(pub ESpriteDirection);

#[derive(Debug, PartialEq)]
pub enum ESpriteDirection {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

impl Default for ESpriteDirection {
    fn default() -> ESpriteDirection {
        ESpriteDirection::RIGHT
    }
}

#[derive(Component)]
pub struct AttackDirection(pub EMovementDirection);

#[derive(Debug, PartialEq)]
pub enum EMovementDirection {
    UP,
    UPRIGHT,
    UPLEFT,
    RIGHT,
    DOWN,
    DOWNRIGHT,
    DOWNLEFT,
    LEFT
}

impl Default for EMovementDirection {
    fn default() -> EMovementDirection {
        EMovementDirection::RIGHT
    }
}

#[derive(Component)]
pub struct CharacterState(pub ECharacterState);


#[derive(Debug, PartialEq)]
pub enum ECharacterState {
    IDLE,
    ATTACK,
    DEAD
}

impl Default for ECharacterState {
    fn default() -> ECharacterState {
        ECharacterState::IDLE
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AttackTimer(pub Timer);

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Projectile(pub ProjectileMask);