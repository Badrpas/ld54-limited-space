use bevy::prelude::*;

use crate::features::player::{unit::UnitSpawnEvent, unit_shoot::AddDamage};

use super::{trigger_hits, UpgradeBundle, WALL_SIZE, HitWall};

pub struct PopulatePlugin;

impl Plugin for PopulatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnWall>();
        app.add_systems(Update, spawn_populate);
        app.add_systems(Update, trigger_hits::<UnitSpawnEvent>);
        app.add_systems(Update, trigger_hits::<AddDamage>);
    }
}

pub enum SpawnWallKind {
    AddUnit { delta: i32 },
    IncDamage { amount: f32 },
}

#[derive(Event)]
pub struct SpawnWall {
    pub at: Vec2,
    pub kind: SpawnWallKind,
}

pub fn spawn_populate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut reader: EventReader<SpawnWall>,
) {
    for info in reader.iter() {
        let mut kek = commands.spawn(UpgradeBundle {
            name: Name::new("Wall"),
            pbr: PbrBundle {
                mesh: meshes.add(shape::Plane::from_size(WALL_SIZE).into()),
                transform: Transform {
                    rotation: Quat::from_rotation_x(std::f32::consts::PI / 2.),
                    ..Transform::from_xyz(info.at.x, 0.5, info.at.y)
                },
                ..default()
            },
        });
        match info.kind {
            SpawnWallKind::AddUnit { delta } => {
                kek.insert(HitWall(UnitSpawnEvent::Delta(delta)));
                kek.insert(materials.add(Color::rgba(0.0, 0.5, 1.3, 0.3).into()));
            }
            SpawnWallKind::IncDamage { amount } => {
                kek.insert(HitWall(AddDamage(amount)));
                kek.insert(materials.add(Color::rgba(1.8, 0.5, 0.3, 0.3).into()));
            }
        };
    }
}
