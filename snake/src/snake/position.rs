use bevy::prelude::*;

use crate::block::UnitBlock;

#[derive(Component, Default, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {}

pub fn corrected(mut query: Query<(&mut Transform, &Position)>, unit_block: Res<UnitBlock>) {
    for (mut transform, position) in &mut query {
        transform.translation.x = position.x * unit_block.0;
        transform.translation.y = position.y * unit_block.0;
    }
}
