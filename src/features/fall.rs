use bevy::prelude::*;

use super::{hp::HitPoints, road::chunk::CHUNK_SIZE};

pub struct FallPlugin;

impl Plugin for FallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fall_system);
    }
}

pub fn fall_system(mut units: Query<(&mut HitPoints, &GlobalTransform)>) {
    for (mut hp, tr) in &mut units {
        let x = tr.translation().x;
        if CHUNK_SIZE / 2. > x && x > -CHUNK_SIZE / 2. {
            continue;
        }

        // log::info!("Despawnint at {x:.2}");
        hp.current = 0.;
    }
}
