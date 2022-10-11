use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Direction(pub EDirection);

#[derive(Debug, PartialEq)]
pub enum EDirection {
    UP,
    LEFT,
    DOWN,
    RIGHT
}

impl Default for EDirection {
    fn default() -> EDirection {
        EDirection::RIGHT
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