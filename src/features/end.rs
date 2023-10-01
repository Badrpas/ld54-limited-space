use bevy::prelude::*;

use super::player::PlayerUnit;

pub struct EndPlugin;

impl Plugin for EndPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, end_system);
    }
}

pub fn end_system(
    units: Query<(), With<PlayerUnit>>,
) {
    if units.is_empty() {
        // log::info!("Le dead");
    }
}

