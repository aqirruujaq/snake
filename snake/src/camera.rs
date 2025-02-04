use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        fn spawn_camera(mut commands: Commands) {
            commands.spawn(Camera2d);
        }
        app.add_systems(Startup, spawn_camera);
    }
}
