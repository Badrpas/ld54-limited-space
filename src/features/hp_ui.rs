use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, *};

use crate::macros::*;

use super::{follow::Follow2d, hp::HitPoints};

pub struct HpUiPlugin;

impl Plugin for HpUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hp_ui_system);
    }
}

#[derive(Component)]
struct HpUiRef(Entity);

fn hp_ui_system(
    mut commands: Commands,
    hps: Query<(Entity, &HitPoints, Option<&HpUiRef>), Changed<HitPoints>>,
    mut uis: Query<(&mut Transform, &mut Visibility, &mut Animator<Transform>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (e, hp, ui_ref) in &hps {
        let ui_e = match ui_ref {
            Some(HpUiRef(entity)) => *entity,
            None => {
                let ui_e = commands
                    .spawn((
                        Follow2d::new().target(e).global(true),
                        SpatialBundle::HIDDEN_IDENTITY,
                        Animator::new(Tween::new(
                            EaseMethod::Linear,
                            Duration::ZERO,
                            TransformScaleLens {
                                start: Vec3::ONE,
                                end: Vec3::ONE,
                            },
                        )),
                    ))
                    .with_children(|cmd| {
                        cmd.spawn(PbrBundle {
                            mesh: meshes.add(shape::Plane::from_size(1.15).into()),
                            material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
                            transform: Transform {
                                rotation: Quat::from_rotation_x(1.),
                                scale: Vec3::new(1., 0.2, 0.25),
                                ..Transform::from_xyz(0., 2.25, 0.2)
                            },
                            ..default()
                        });
                        cmd.spawn(PbrBundle {
                            mesh: meshes.add(shape::Plane::from_size(1.).into()),
                            material: materials.add(Color::rgb(0.2, 0.99, 0.3).into()),
                            transform: Transform {
                                rotation: Quat::from_rotation_x(1.),
                                scale: Vec3::new(1., 0.2, 0.2),
                                ..Transform::from_xyz(0., 2.3, 0.3)
                            },
                            ..default()
                        });
                    })
                    .id();
                commands.entity(e).insert(HpUiRef(ui_e));
                ui_e
            }
        };

        let (ui, mut v, mut a) = ok_or_skip!(uis.get_mut(ui_e));

        a.set_tweenable(Tween::new(
            // Use a quadratic easing on both endpoints.
            EaseFunction::QuadraticInOut,
            // Animation time (one way only; for ping-pong it takes 2 seconds
            // to come back to start).
            Duration::from_secs_f32(0.1),
            TransformScaleLens {
                start: ui.scale.clone(),
                end: {
                    let mut scale = ui.scale.clone();
                    scale.x = hp.current / hp.max;
                    scale
                },
            },
        ));

        if hp.current / hp.max >= 0.99 {
            *v = Visibility::Hidden;
        } else {
            *v = Visibility::Visible;
        }
    }
}
