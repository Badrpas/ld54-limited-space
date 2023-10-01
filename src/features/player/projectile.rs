use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{
    features::{
        damage::DamageEntry,
        follow::{Follow2d, Follow2dKind, FollowTarget},
        hp::HitPoints,
    },
    macros::*,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProjectileSpawn>();
        app.add_systems(Update, (projectile_spawn, projectile_update));
    }
}

#[derive(Event)]
pub struct ProjectileSpawn {
    pub target: Entity,
    pub damage: f32,
    pub from: Vec3,
}

#[derive(Component)]
pub struct Projectile;

fn projectile_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut reader: EventReader<ProjectileSpawn>,
) {
    for ev in reader.iter() {
        commands.spawn((
            HitPoints::new(ev.damage),
            Follow2d::new()
                .target(ev.target)
                .kind(Follow2dKind::Linear { speed: 40. })
                .auto_despawn(true),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                material: materials.add(Color::rgb(2.9, 2.9, 0.2).into()),
                transform: Transform::from_translation(ev.from),
                ..default()
            },
        ));
    }
}

fn projectile_update(
    projectiles: Query<(Entity, &Transform, &Follow2d, &HitPoints)>,
    trs: Query<&Transform>,
    mut damages: EventWriter<DamageEntry>,
) {
    for (projectile, tr, follow, hp) in &projectiles {
        let target_e = get_or_skip!(FollowTarget::Entity(e) = follow.target => e);
        let target = ok_or_skip!(trs.get(target_e));
        if target.translation.zx().distance(tr.translation.zx()) < 0.5 {
            damages.send(DamageEntry {
                target: target_e,
                amount: hp.max,
            });
            damages.send(DamageEntry {
                target: projectile,
                amount: hp.max,
            });
        }
    }
}
