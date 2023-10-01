use bevy::prelude::*;

use crate::{features::{road::enemy::{Speed, DamageInvoker}, player::{PlayerUnit, PlayerRoot, unit::get_unit_radius}, hp::HitPoints}, macros::*};

pub struct SlugPlugin;

impl Plugin for SlugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, slug_system);
    }
}

#[derive(Component)]
pub struct Slug;


pub fn slug_system(
    time: Res<Time>,
    root: Query<&Transform, (With<PlayerRoot>, Without<Speed>)>,
    units: Query<(&GlobalTransform, Entity), With<PlayerUnit>>,
    mut npcs: Query<(&mut Transform, &Speed, &mut DamageInvoker)>,
    mut hps: Query<&mut HitPoints>,
) {
    let root = ok_or_ret!(root.get_single());
    let radius = get_unit_radius(units.iter().len());
    for (mut transform, speed, mut di) in npcs.iter_mut() {
        let diff = root.translation - transform.translation;
        if diff.length() <= radius + 1. {
            // find closest and then attaack or follow it
            let closest = some_or_skip!(units.iter().min_by(|a, b| {
                transform.translation.distance_squared(a.0.translation())
                    .total_cmp(&transform.translation.distance_squared(b.0.translation()))
            }));
            let mut diff = closest.0.translation() - transform.translation;
            diff.y = 0.;
            if diff.length() <= 1. {
                if di.invoke() {
                    log::info!("Do damage");
                    let mut hp = ok_or_skip!(hps.get_mut(closest.1));
                    hp.current -= di.amount;
                }
            } else {
                let dir = diff.normalize();
                transform.translation += dir * time.delta_seconds() * **speed;
            }
        } else {
            // go in general direction
            let dir = diff.normalize_or_zero();
            let diff = dir * time.delta_seconds() * **speed;
            transform.translation += diff;
        }
    }
}

