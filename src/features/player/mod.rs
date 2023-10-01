pub mod camera_follow;
pub mod controls;
pub mod unit;
pub mod unit_shoot;
pub mod projectile;
use bevy::prelude::*;

use self::unit::UnitSpawnEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player);
        app.add_plugins(controls::ControlsPlugin);
        app.add_plugins(camera_follow::CameraFollowPlugin);
        app.add_plugins(unit::UnitPlugin);
        app.add_plugins(unit_shoot::UnitShootPlugin);
        app.add_plugins(projectile::ProjectilePlugin);
    }
}

#[derive(Component)]
pub struct PlayerRoot;
#[derive(Component)]
pub struct PlayerUnit;

pub fn init_player(mut commands: Commands, mut unit_ev: EventWriter<UnitSpawnEvent>) {
    commands.spawn((
        Name::new("Player Root"),
        PlayerRoot,
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..default()
        },
    ));
    unit_ev.send(UnitSpawnEvent::Delta(1));
}
