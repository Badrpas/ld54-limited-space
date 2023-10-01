use bevy::prelude::*;

use crate::features::road::chunk::CHUNK_SIZE;

use super::{PlayerRoot, PlayerUnit};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, controls_system);
    }
}

fn controls_system(
    time: Res<Time>,
    kb: Res<Input<KeyCode>>,
    mut root: Query<&mut Transform, With<PlayerRoot>>,
    units: Query<(), With<PlayerUnit>>,
) {
    if units.is_empty() { return; }
    let mut dir = Vec2::splat(0.);
    if kb.pressed(KeyCode::D) {
        dir.x += 1.;
    }
    if kb.pressed(KeyCode::A) {
        dir.x -= 1.;
    }

    dir.y -= 1.; // autorun

    let mut dir = dir.normalize_or_zero();
    if dir.length_squared() < 0.01 {
        return;
    }

    if kb.pressed(KeyCode::W) {
        dir.y -= 0.5;
    }
    if kb.pressed(KeyCode::S) {
        dir.y += 0.3;
    }

    let mut diff = dir * time.delta_seconds() * 5.;

    if let Ok(mut root) = root.get_single_mut() {
        const LIMIT: f32 = CHUNK_SIZE / 2. - 0.1;
        if diff.x > 0. && root.translation.x + diff.x >= LIMIT {
            diff.x = LIMIT - root.translation.x;
        }
        if diff.x < 0. && root.translation.x + diff.x <= -LIMIT {
            diff.x = -LIMIT - root.translation.x;
        }
        root.translation.x += diff.x;
        root.translation.z += diff.y;
    }
}
