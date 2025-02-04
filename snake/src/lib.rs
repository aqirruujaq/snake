//SCREEN_BLOCK_LENGTH has to be odd
const SCREEN_BLOCK_LENGTH: i32 = 9;
const SCREEN_COLOR: Color = Color::WHITE;
const MOVEMNET_CYCLE: f64 = 0.5;

pub mod block;
mod camera;
mod score;
mod snake;

use bevy::prelude::*;

use block::BlockPlugin;
use camera::CameraPlugin;
use score::ScorePlugin;
use snake::SnakePlugin;

pub fn run() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            BlockPlugin,
            SnakePlugin,
            ScorePlugin,
        ))
        .run();
}
