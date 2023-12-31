use bevy_rapier3d::prelude::*;
use std::time::{Duration, Instant};

use bevy::prelude::*;

use crate::{
    features::{
        follow::Follow2d,
        hp::HitPoints,
        hp_ui::HUD,
        player::{unit::ShootInfo, unit_shoot::UnitShooting, PlayerRoot, PlayerUnit},
        road::chunk::CHUNK_SIZE,
        team::Team,
    },
    macros::*,
    util::sunflower::sunflower,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy);
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct Speed(f32);

#[derive(Component)]
pub struct DamageInvoker {
    pub last_invoke: Instant,
    pub cooldown: Duration,
    pub amount: f32,
}

impl DamageInvoker {
    pub fn new(amount: f32, cd: f32) -> Self {
        Self {
            last_invoke: Instant::now(),
            cooldown: Duration::from_secs_f32(cd),
            amount,
        }
    }
    pub fn is_ready(&self) -> bool {
        self.last_invoke + self.cooldown < Instant::now()
    }
    pub fn invoke(&mut self) -> bool {
        if self.is_ready() {
            self.last_invoke = Instant::now();
            true
        } else {
            false
        }
    }
}

pub fn spawn_enemy(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut extra_power: Local<f32>,
    us: Res<UnitShooting>,
    player_root: Query<&Transform, With<PlayerRoot>>,
    npcs: Query<(&Speed, &DamageInvoker, &HitPoints), With<Enemy>>,
    units: Query<&ShootInfo, With<PlayerUnit>>,
) {
    *extra_power += time.delta_seconds() * 0.2;
    let player_tr = ok_or_ret!(player_root.get_single());
    let player_power: f32 = units.iter().fold(0., |acc, info| {
        acc + (us.damage / info.cooldown.as_secs_f32()) * (us.range / 5.)
    }) + *extra_power;
    let mut enemy_power: f32 = npcs
        .iter()
        .map(|(s, di, hp)|(**s, di.amount, di.cooldown.as_secs_f32(), hp.max))
        .map(npc_power)
        .reduce(|a, x| a + x)
        .unwrap_or_default();

    fn npc_power((speed, dmg, cd, hp): (f32, f32, f32, f32)) -> f32 {
        (dmg / cd) + (speed / 3.) + (hp / 100.)
    }

    let diff = player_power - enemy_power;

    while diff > 1. && player_power >= enemy_power {
        let (x, y) = sunflower(
            enemy_power as usize,
            player_power as usize,
            2.0,
            CHUNK_SIZE / 2. - 0.05,
        );

        let speed = 6. + diff * 0.01;
        let damage = 0.1 + diff * 0.03;
        let cd = 0.2;
        let hp = 10. + 10. * diff * 0.01;

        commands
            .spawn((
                (
                    RigidBody::Dynamic,
                    Collider::ball(0.5),
                    Restitution::coefficient(0.0),
                    LockedAxes::TRANSLATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED,
                ),
                Name::new("NPC"),
                Speed(speed),
                DamageInvoker::new(damage, cd),
                Enemy,
                Team::Enemy,
                HUD,
                HitPoints::new(hp),
                Follow2d::new(),
                SpatialBundle {
                    transform: Transform::from_xyz(
                        x,
                        0.0,
                        y + player_tr.translation.z - CHUNK_SIZE * 4.,
                    ),
                    ..default()
                },
            ))
            .with_children(|cmd| {
                cmd.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                    material: materials.add(Color::rgb(0.9, 0.2, 0.2).into()),
                    transform: Transform::from_xyz(0.0, 1.0, 0.0),
                    ..default()
                });
            });

        let added = npc_power((speed, damage, cd, hp));
        log::info!("diff: {diff:.2} power {added:.2} speed {speed:.2} damage {damage:.2} cd {cd:.2} hp {hp:.2}");
        enemy_power += added;
    }
}
