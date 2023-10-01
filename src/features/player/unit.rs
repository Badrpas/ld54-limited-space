use bevy::prelude::*;
use rand::Rng;

use crate::{
    features::{
        follow::{Follow2d, FollowTarget, Follow2dKind},
        hp::HitPoints,
        player::PlayerUnit,
        road::chunk::CHUNK_SIZE,
        team::Team,
    },
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
                    PlayerUnit,
                    Follow2d::new().kind(Follow2dKind::Exponential { seconds: 0.3 }),
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                        material: materials.add(Color::rgb(r, g, b).into()),
                        transform: Transform::from_xyz(0.0, 1.0, 0.0),
                        ..default()
                    },
                ))
                .set_parent(root);
        }
    }
}

pub fn get_unit_radius(n: usize) -> f32 {
    CHUNK_SIZE.min((n as f32).sqrt() / 2.)
}

fn reposition_units(mut units: Query<&mut Follow2d, With<PlayerUnit>>) {
    let count = units.iter().len();
    let max_radius = get_unit_radius(count);
    for (i, mut unit) in units.iter_mut().enumerate() {
        let (x, y) = sunflower(i + 1, count, 2.0, max_radius);
        unit.target = Vec3::new(x, 0., y).into();
    }
}
