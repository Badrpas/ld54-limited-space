use bevy::prelude::*;

use crate::macros::*;

use super::hp::HitPoints;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEntry>();
        app.add_systems(Update, damage_system);
    }
}

#[derive(Event)]
pub struct DamageEntry {
    pub target: Entity,
    pub amount: f32,
}

pub fn damage_system(
    mut entries: EventReader<DamageEntry>,
    mut hps: Query<&mut HitPoints>,
) {
    for entry in entries.iter() {
        let mut hp = ok_or_skip!(hps.get_mut(entry.target));
        hp.current -= entry.amount.min(hp.current);
    }
}

