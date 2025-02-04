use bevy::{prelude::*, window::WindowResized};

use crate::{SCREEN_BLOCK_LENGTH, SCREEN_COLOR};

#[derive(Component, Default)]
pub struct Block(pub f32);

#[derive(Resource)]
pub struct UnitBlock(pub f32);

impl UnitBlock {
    fn update(
        mut resize_reader: EventReader<WindowResized>,
        mut unit_block: ResMut<UnitBlock>,
        mut query: Query<(&mut Transform, &Block)>,
    ) {
        for e in resize_reader.read() {
            unit_block.0 = e.width.min(e.height) / SCREEN_BLOCK_LENGTH as f32;

            for (mut transform, block) in &mut query {
                transform.scale.x = unit_block.0 * block.0;
                transform.scale.y = unit_block.0 * block.0;
            }
        }
    }
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitBlock(0.0))
            .add_systems(Startup, spawn_screen)
            .add_systems(PreUpdate, UnitBlock::update);
    }
}

fn spawn_screen(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: SCREEN_COLOR,
            ..default()
        },
        Block(SCREEN_BLOCK_LENGTH as f32),
        Transform::default(),
    ));
}
