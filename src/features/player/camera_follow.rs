use bevy::prelude::*;

use crate::features::follow::{Follow2d, Follow2dKind, FollowTarget};

use super::PlayerRoot;

pub struct CameraFollowPlugin;

impl Plugin for CameraFollowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Initialized>();
        app.add_systems(Update, init_camera_follow.run_if(condition_met));
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Initialized(bool);

fn condition_met(
    init: Res<Initialized>,
    root: Query<Entity, With<PlayerRoot>>,
    camera: Query<Entity, With<Camera>>,
) -> bool {
    return !(**init) && !root.is_empty() && !camera.is_empty();
}

fn init_camera_follow(
    mut init: ResMut<Initialized>,
    mut commands: Commands,
    root: Query<Entity, With<PlayerRoot>>,
    camera: Query<Entity, With<Camera>>,
) {
    log::info!("Initializing");
    let root = root.get_single().unwrap();
    let camera_mount = commands
        .spawn((
            Name::new("Camera mount point"),
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 12.5, 15.0)),
        ))
        .set_parent(root)
        .id();

    if let Ok(camera) = camera.get_single() {
        commands.entity(camera).insert(
            Follow2d::new()
                .target(camera_mount)
                .kind(Follow2dKind::Exponential { seconds: 0.4 })
                .global(true),
        );
        **init = true;
    } else {
        log::error!("No bueno camera not found");
    }
}
