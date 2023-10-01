use bevy::{math::Vec3Swizzles, prelude::*};

use crate::macros::*;

pub struct FollowPlugin;

impl Plugin for FollowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (follow_diff_calc, move_followers).chain());
    }
}

#[derive(Reflect)]
pub enum FollowTarget {
    None,
    Entity(Entity),
    Vec(Vec3),
}

#[derive(Component, Reflect)]
pub struct Follow2d {
    pub lag_seconds: f32,
    pub target: FollowTarget,
    pub target_global: bool,
    pub diff: Vec2,
    pub leeway: f32,
}
impl Follow2d {
    pub fn new() -> Self {
        Self {
            lag_seconds: 0.3,
            target: FollowTarget::None,
            target_global: false,
            diff: default(),
            leeway: default(),
        }
    }
    pub fn target(self, val: FollowTarget) -> Self {
        Self {
            target: val,
            ..self
        }
    }
    pub fn target_global(self, val: bool) -> Self {
        Self {
            target_global: val,
            ..self
        }
    }
}

pub fn follow_diff_calc(
    time: Res<Time>,
    mut followers: Query<(&Transform, &mut Follow2d)>,
    transforms: Query<(&Transform, &GlobalTransform)>,
) {
    for (tr, mut follow) in followers.iter_mut() {
        let target = {
            match follow.target {
                FollowTarget::Entity(target) => {
                    let (target, target_g) = ok_or_skip!(transforms.get(target));
                    {
                        if follow.target_global {
                            target_g.translation()
                        } else {
                            target.translation
                        }
                    }
                    .xz()
                }
                FollowTarget::Vec(tr) => tr.xz(),
                FollowTarget::None => continue,
            }
        };

        let delta = follow_exp_diff(
            tr.translation.xz(),
            target,
            follow.lag_seconds,
            time.delta_seconds(),
            follow.leeway,
        );
        if delta.length() > 0.01 {
            follow.diff = delta;
        }
    }
}

pub fn follow_exp_diff(source: Vec2, target: Vec2, seconds: f32, dt: f32, leeway: f32) -> Vec2 {
    let diff = target - source;
    let diff_length = diff.length();
    let needed_length = (diff_length - leeway).max(0.);
    let dir = diff.normalize_or_zero();
    let speed = needed_length / seconds;
    let step = speed * dt;
    if step >= 0.01 {
        dir * step
    } else {
        Vec2::ZERO
    }
}

fn move_followers(
    // time: Res<Time>,
    mut q: Query<(&mut Transform, &mut Follow2d)>,
) {
    for (mut transform, mut fe) in &mut q {
        transform.translation += Vec3::new(fe.diff.x, 0., fe.diff.y);
        fe.diff.x = 0.;
        fe.diff.y = 0.;
    }
}
