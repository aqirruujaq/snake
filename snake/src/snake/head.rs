use bevy::prelude::*;

use super::food::{Food, SpawnFood};
use super::position::Position;
use crate::block::Block;

#[derive(Component)]
pub struct Head;

pub fn spawn(mut commands: Commands, mut event: EventWriter<SpawnFood>) {
    commands.spawn((
        Head,
        Block(1.0),
        Position::default(),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.3),
            ..default()
        },
        Sprite {
            color: Color::BLACK,
            ..default()
        },
    ));
    event.send_default();
}

//It will be update at the end
pub fn snake_movement(
    mut single: Single<(&mut Transform, &mut Position), With<Head>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyW) {
        single.1.y += 1.0;
    }
    if input.just_pressed(KeyCode::KeyS) {
        single.1.y -= 1.0;
    }
    if input.just_pressed(KeyCode::KeyD) {
        single.1.x += 1.0;
    }
    if input.just_pressed(KeyCode::KeyA) {
        single.1.x -= 1.0;
    }
}

pub fn eat_food(
    mut commands: Commands,
    head: Single<&Position, With<Head>>,
    option: Option<Single<(&Position, Entity), With<Food>>>,
    mut event: EventWriter<SpawnFood>,
) {
    if let Some(food) = option {
        if head.x == food.0.x && head.y == food.0.y {
            commands.entity(food.1).despawn();
            event.send_default();
        }
    }
}
