use bevy::prelude::*;

use crate::{features::road::enemy::Enemy, macros::*};

use super::{projectile::ProjectileSpawn, unit::ShootInfo, PlayerUnit};

pub struct UnitShootPlugin;

impl Plugin for UnitShootPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitShooting {
            range: 9.,
            damage: 1.,
        });
        app.add_event::<AddDamage>();
        app.add_systems(Update, (unit_shoot_system, increase_damage));
    }
}

#[derive(Resource)]
pub struct UnitShooting {
    pub range: f32,
    pub damage: f32,
}

#[derive(Event, Clone)]
pub struct AddDamage(pub f32);

pub fn increase_damage(mut us: ResMut<UnitShooting>, mut reader: EventReader<AddDamage>) {
    for ev in reader.iter() {
        us.damage += ev.0;
    }
}

pub fn unit_shoot_system(
    time: Res<Time>,
    us: Res<UnitShooting>,
    mut units: Query<(&GlobalTransform, &mut ShootInfo), With<PlayerUnit>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut projectiles_bus: EventWriter<ProjectileSpawn>,
) {
    for (unit, mut shoot_info) in &mut units {
        if !shoot_info.is_ready(&time) {
            continue;
        }
        let (closest_e, closest_tr) = some_or_skip!(enemies.iter().min_by(|a, b| {
            unit.translation()
                .distance_squared(a.1.translation)
                .total_cmp(&unit.translation().distance_squared(b.1.translation))
        }));
        if closest_tr.translation.distance(unit.translation()) > us.range {
            continue;
        }
        if shoot_info.try_shoot(&time) {
            projectiles_bus.send(ProjectileSpawn {
                target: closest_e,
                damage: us.damage,
                from: unit.translation(),
            });
        }
    }
}
