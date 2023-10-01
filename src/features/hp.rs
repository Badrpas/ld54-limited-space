use bevy::prelude::*;

pub struct HpPlugin;

impl Plugin for HpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hp_system);
    }
}

#[derive(Component)]
pub struct HitPoints {
    pub current: f32,
    pub max: f32,
}

impl HitPoints {
    pub fn new(hp: f32) -> Self {
        Self {
            current: hp,
            max: hp,
        }
    }
}

pub fn hp_system(
    mut commands: Commands,
    units: Query<(Entity, &HitPoints)>,
) {
    for (unit, hp) in &units {
        if hp.current <= 0. {
            commands.entity(unit).despawn_recursive();
        }
    }
}

