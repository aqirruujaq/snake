mod body;
mod food;
mod head;
mod position;

use bevy::prelude::*;
use food::{spawn_food, SpawnFood};

use crate::MOVEMNET_CYCLE;
use head::{eat_food, snake_movement, spawn};
use position::corrected;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_seconds(MOVEMNET_CYCLE))
            .add_event::<SpawnFood>()
            .add_systems(Startup, spawn)
            .add_systems(
                /*FixedUpdate*/ Update,
                (snake_movement, spawn_food, eat_food, corrected).chain(),
            );
    }
}
