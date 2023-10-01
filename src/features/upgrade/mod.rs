use self::populate::SpawnWall;

use super::{
    player::{unit::get_unit_radius, PlayerRoot, PlayerUnit},
    road::chunk::{RoadChunk, CHUNK_SIZE},
};
use crate::macros::*;
use bevy::{math::Vec3Swizzles, prelude::*};
use rand::Rng;
pub mod populate;

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(populate::PopulatePlugin);
        app.add_systems(Update, on_chunk);
    }
}

#[derive(Component)]
pub struct Link {
    pub upgrades: Vec<Entity>,
}

#[derive(Component)]
pub struct HitWall<E: Event>(E);

#[derive(Bundle)]
pub struct UpgradeBundle {
    pub name: Name,
    pub pbr: PbrBundle,
}

const WALL_SIZE: f32 = 3.;

fn trigger_hits<E: Event + Clone>(
    mut commands: Commands,
    mut writer: EventWriter<E>,
    player: Query<&Transform, With<PlayerRoot>>,
    units: Query<(), With<PlayerUnit>>,
    walls: Query<(Entity, &Transform, &HitWall<E>)>,
) {
    let player = ok_or_ret!(player.get_single());
    let radius = get_unit_radius(units.iter().len());
    for (e, tr, wall) in &walls {
        if tr.translation.distance(player.translation) < radius.max(WALL_SIZE / 2.) {
            writer.send(wall.0.clone());
            commands.entity(e).despawn_recursive();
        }
    }
}

fn on_chunk(new_chunks: Query<&Transform, Added<RoadChunk>>, mut populate: EventWriter<SpawnWall>) {
    let max_shift = CHUNK_SIZE / 2. - WALL_SIZE / 2.;
    for chunk in &new_chunks {
        let kind = match rand::thread_rng().gen_range(0..=1) {
            0 => populate::SpawnWallKind::AddUnit { delta: 2 },
            1 => populate::SpawnWallKind::IncDamage { amount: 1. },
            _ => populate::SpawnWallKind::AddUnit { delta: 1 },
        };
        populate.send(SpawnWall {
            kind,
            at: chunk.translation.xz()
                + Vec2::X * rand::thread_rng().gen_range(-max_shift..max_shift),
        });
    }
}
