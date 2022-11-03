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

impl Default for ESpriteDirection {
    fn default() -> ESpriteDirection {
        ESpriteDirection::DOWN
    }
}

#[derive(Debug, PartialEq)]
pub enum ESpriteDirection {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

#[derive(Debug, Component)]
pub struct CharacterState(pub (ECharacterMovementState, ECharacterAttackState));

#[derive(Debug, PartialEq)]
pub enum ECharacterMovementState {
    IDLE,
    WALK_UP,
    WALK_RIGHT,
    WALK_DOWN,
    WALK_LEFT
}

impl Default for ECharacterMovementState {
    fn default() -> ECharacterMovementState {
        ECharacterMovementState::IDLE
    }
}

#[derive(Debug, PartialEq)]
pub enum ECharacterAttackState {
    IDLE,
    ATTACK_UP,
    ATTACK_UPRIGHT,
    ATTACK_UPLEFT,
    ATTACK_RIGHT,
    ATTACK_DOWN,
    ATTACK_DOWNRIGHT,
    ATTACK_DOWNLEFT,
    ATTACK_LEFT,
}

impl Default for ECharacterAttackState {
    fn default() -> ECharacterAttackState {
        ECharacterAttackState::IDLE
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
