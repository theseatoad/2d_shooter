use std::time::Duration;

use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    components::{
        AnimationTimer, CharacterState, ECharacterAttackState, ECharacterMovementState,
        ESpriteDirection, SpriteDirection,
    },
    player::{ATTACK_ANIM_SPEED, IDLE_ANIM_SPEED},
    GameState, TIME_STEP,
};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(change_sprite.before(animate_sprite))
                .with_system(animate_sprite),
        );
    }
}
pub const DOWN_ATTACK_0: usize = 0;
pub const DOWN_ATTACK_1: usize = 1;
pub const LEFT_ATTACK_0: usize = 2;
pub const LEFT_ATTACK_1: usize = 3;
pub const RIGHT_ATTACK_0: usize = 4;
pub const RIGHT_ATTACK_1: usize = 5;
pub const UP_ATTACK_0: usize = 6;
pub const UP_ATTACK_1: usize = 7;

//pub const DEAD_0: usize = 8;

pub const DOWN_IDLE_0: usize = 9;
pub const DOWN_IDLE_1: usize = 10;
pub const LEFT_IDLE_0: usize = 11;
pub const LEFT_IDLE_1: usize = 12;
pub const RIGHT_IDLE_0: usize = 13;
pub const RIGHT_IDLE_1: usize = 14;
pub const UP_IDLE_0: usize = 15;
pub const UP_IDLE_1: usize = 16;

fn change_sprite(
    mut query: Query<(&mut SpriteDirection, &CharacterState), Changed<CharacterState>>,
) {
    for (mut sprite_direction, character_state) in &mut query {
        // Attacking is higher priority over idle
        if character_state.0 .1 != ECharacterAttackState::IDLE {
            match character_state.0 .1 {
                ECharacterAttackState::ATTACK_UP => sprite_direction.0 = ESpriteDirection::UP,
                ECharacterAttackState::ATTACK_UPRIGHT => {
                    sprite_direction.0 = ESpriteDirection::RIGHT
                }
                ECharacterAttackState::ATTACK_UPLEFT => sprite_direction.0 = ESpriteDirection::LEFT,
                ECharacterAttackState::ATTACK_RIGHT => sprite_direction.0 = ESpriteDirection::RIGHT,
                ECharacterAttackState::ATTACK_DOWN => sprite_direction.0 = ESpriteDirection::DOWN,
                ECharacterAttackState::ATTACK_DOWNRIGHT => {
                    sprite_direction.0 = ESpriteDirection::RIGHT
                }
                ECharacterAttackState::ATTACK_DOWNLEFT => {
                    sprite_direction.0 = ESpriteDirection::LEFT
                }
                ECharacterAttackState::ATTACK_LEFT => sprite_direction.0 = ESpriteDirection::LEFT,
                ECharacterAttackState::IDLE => { // Should not get here
                }
            }
        } else if character_state.0 .0 != ECharacterMovementState::IDLE {
            match character_state.0 .0 {
                ECharacterMovementState::WALK_DOWN => sprite_direction.0 = ESpriteDirection::DOWN,
                ECharacterMovementState::WALK_LEFT => sprite_direction.0 = ESpriteDirection::LEFT,
                ECharacterMovementState::WALK_RIGHT => sprite_direction.0 = ESpriteDirection::RIGHT,
                ECharacterMovementState::WALK_UP => sprite_direction.0 = ESpriteDirection::UP,
                ECharacterMovementState::IDLE => {
                    //Should not get here
                }
            }
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &CharacterState,
        &SpriteDirection,
    )>,
) {
    for (mut timer, mut sprite, character_state, sprite_direction) in &mut query {
        let mut next_sprite = false;
        timer.tick(time.delta());
        if timer.just_finished() {
            next_sprite = true;
        }
        let ( first_index, second_index);
        // Settings the first and second index for sprite direction.
        if character_state.0 .1 != ECharacterAttackState::IDLE {
            match sprite_direction.0 {
                ESpriteDirection::UP => (first_index, second_index) = (UP_ATTACK_0, UP_ATTACK_1),
                ESpriteDirection::RIGHT => {
                    (first_index, second_index) = (RIGHT_ATTACK_0, RIGHT_ATTACK_1)
                }
                ESpriteDirection::DOWN => {
                    (first_index, second_index) = (DOWN_ATTACK_0, DOWN_ATTACK_1)
                }
                ESpriteDirection::LEFT => {
                    (first_index, second_index) = (LEFT_ATTACK_0, LEFT_ATTACK_1)
                }
            }
        } else {
            match sprite_direction.0 {
                ESpriteDirection::UP => (first_index, second_index) = (UP_IDLE_0, UP_IDLE_1),
                ESpriteDirection::RIGHT => {
                    (first_index, second_index) = (RIGHT_IDLE_0, RIGHT_IDLE_1)
                }
                ESpriteDirection::DOWN => (first_index, second_index) = (DOWN_IDLE_0, DOWN_IDLE_1),
                ESpriteDirection::LEFT => (first_index, second_index) = (LEFT_IDLE_0, LEFT_IDLE_1),
            }
        }
        // Sets other animations to this current one
        if sprite.index != first_index && sprite.index != second_index {
            timer.reset();
            sprite.index = first_index;
            // Make sure we are getting the correct animation speed for our timer.
            if character_state.0.1 != ECharacterAttackState::IDLE {
                timer.set_duration(Duration::from_secs_f32(ATTACK_ANIM_SPEED));
            } else {
                timer.set_duration(Duration::from_secs_f32(IDLE_ANIM_SPEED));
            }
            //If is the first one and it is ready to go to next
        } else if sprite.index != first_index && next_sprite {
            sprite.index = first_index;
            // If it is the second one ready to go to the first.
        } else if sprite.index == first_index && next_sprite {
            sprite.index = second_index;
        }
    }
}
