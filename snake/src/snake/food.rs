use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{block::Block, score::Score, SCREEN_BLOCK_LENGTH};

use super::position::Position;

#[derive(Component)]
pub struct Food;

#[derive(Event, Default)]
pub struct SpawnFood;

pub fn spawn_food(
    mut commands: Commands,
    pos: Query<&Position>,
    mut event: EventReader<SpawnFood>,
) {
    for _ in event.read() {
        print!("123");
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(-SCREEN_BLOCK_LENGTH / 2..SCREEN_BLOCK_LENGTH / 2) as f32;
        let mut y = rng.gen_range(-SCREEN_BLOCK_LENGTH / 2..SCREEN_BLOCK_LENGTH / 2) as f32;

        loop {
            for p in &pos {
                if p.x == x && p.y == y {
                    x = rng.gen_range(-SCREEN_BLOCK_LENGTH / 2..SCREEN_BLOCK_LENGTH / 2) as f32;
                    y = rng.gen_range(-SCREEN_BLOCK_LENGTH / 2..SCREEN_BLOCK_LENGTH / 2) as f32;
                    continue;
                }
            }
            break;
        }

        commands.spawn((
            Food,
            Block(0.9),
            Position { x, y },
            Transform {
                translation: Vec3::new(0.0, 0.0, 0.2),
                ..default()
            },
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                ..default()
            },
        ));
    }
}
