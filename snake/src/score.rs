use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub i32);

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
    }
}
