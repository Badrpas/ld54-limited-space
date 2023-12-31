use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_system);
    }
}

pub fn camera_system(
    mut commands: Commands,
) {
    // camera
    commands.spawn(Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 12.5, 15.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

