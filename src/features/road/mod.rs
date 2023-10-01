pub mod chunk;
pub mod enemy;
use bevy::prelude::*;

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(chunk::RoadChunkPlugin);
        app.add_plugins(enemy::EnemyPlugin);
    }
}

