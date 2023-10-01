use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_tweening::{lens::TransformScaleLens, *};
use rand::Rng;

use crate::{
    features::{
        follow::{Follow2d, Follow2dKind, FollowTarget},
        hp::HitPoints,
        player::PlayerUnit,
        road::chunk::CHUNK_SIZE,
        team::Team, hp_ui::HUD,
    },
    macros::*,
    util::sunflower::sunflower,
};

use super::PlayerRoot;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UnitSpawnEvent>();
        app.add_systems(
            Update,
            (trigger_add_unit, add_unit, reposition_units).chain(),
        );
    }
}

#[derive(Event, PartialEq, Eq)]
pub enum UnitSpawnEvent {
    New,
}

#[derive(Component)]
pub struct ShootInfo {
    pub last_shot: Duration,
    pub cooldown: Duration,
}

impl ShootInfo {
    pub fn new(cooldown_seconds: f32) -> Self {
        let cooldown = Duration::from_secs_f32(cooldown_seconds);
        Self {
            last_shot: Duration::ZERO,
            cooldown,
        }
    }
    pub fn is_ready(&self, time: &Res<Time>) -> bool {
        time.elapsed() >= self.last_shot + self.cooldown
    }
    pub fn try_shoot(&mut self, time: &Res<Time>) -> bool {
        if self.is_ready(time) {
            self.last_shot = time.elapsed();
            return true;
        }
        false
    }
}

fn trigger_add_unit(kb: Res<Input<KeyCode>>, mut ev_writer: EventWriter<UnitSpawnEvent>) {
    if kb.just_pressed(KeyCode::M) {
        ev_writer.send(UnitSpawnEvent::New);
    }
}

fn add_unit(
    root: Query<Entity, With<PlayerRoot>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ev_reader: EventReader<UnitSpawnEvent>,
) {
    for event in ev_reader.iter() {
        if event != &UnitSpawnEvent::New {
            continue;
        }
        if let Ok(root) = root.get_single() {
            log::info!("Adding unit");
            let r = rand::thread_rng().gen_range(0.2..0.9);
            let g = rand::thread_rng().gen_range(0.2..0.9);
            let b = rand::thread_rng().gen_range(0.2..0.9);
            commands
                .spawn((
                    Team::Player,
                    HitPoints::new(10.),
                    HUD,
                    PlayerUnit,
                    Follow2d::new().kind(Follow2dKind::Exponential { seconds: 0.3 }),
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                        material: materials.add(Color::rgb(r, g, b).into()),
                        transform: Transform::from_xyz(0.0, 1.0, 0.0),
                        ..default()
                    },
                    ShootInfo::new(0.7),
                ))
                .set_parent(root);
        }
    }
}

pub fn get_unit_radius(n: usize) -> f32 {
    CHUNK_SIZE.min((n as f32).sqrt() / 2.)
}

fn reposition_units(
    mut last_count: Local<usize>,
    mut buf: Local<VecDeque<(f32, f32)>>,
    mut units: Query<(&mut Follow2d, &Transform), With<PlayerUnit>>,
) {
    let count = units.iter().len();
    if *last_count == count {
        return;
    }
    *last_count = count;

    let max_radius = get_unit_radius(count);

    buf.clear();
    for i in 0..count {
        buf.push_front(sunflower(i, count, 2.0, max_radius));
    }

    for (mut unit, tr) in units.iter_mut() {
        let unit_pos = tr.translation.xz();
        let (index, &(x, y)) = some_or_skip!(buf.iter().enumerate().min_by(|a, b| {
            unit_pos
                .distance_squared((*a.1).into())
                .total_cmp(&unit_pos.distance_squared((*b.1).into()))
        }));
        buf.remove(index);
        unit.target = Vec3::new(x, 0., y).into();
    }
}
