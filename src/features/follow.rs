use std::ops::Mul;

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
impl From<Entity> for FollowTarget {
    fn from(value: Entity) -> Self {
        FollowTarget::Entity(value)
    }
}
impl From<()> for FollowTarget {
    fn from(_: ()) -> Self {
        FollowTarget::None
    }
}
impl From<Vec3> for FollowTarget {
    fn from(value: Vec3) -> Self {
        FollowTarget::Vec(value)
    }
}

#[derive(Reflect)]
pub enum Follow2dKind {
    Exponential { seconds: f32 },
    Linear { speed: f32 },
    Snap,
}

#[derive(Component, Reflect)]
pub struct Follow2d {
    pub kind: Follow2dKind,
    pub target: FollowTarget,
    pub target_global: bool,
    pub diff: Vec2,
    pub leeway: f32,
}
impl Follow2d {
    pub fn new() -> Self {
        Self {
            kind: Follow2dKind::Snap,
            target: FollowTarget::None,
            target_global: false,
            diff: default(),
            leeway: default(),
        }
    }
    pub fn target<F: Into<FollowTarget>>(mut self, val: F) -> Self {
        self.target = val.into();
        self
    }
    pub fn global(mut self, val: bool) -> Self {
        self.target_global = val;
        self
    }
    pub fn kind(mut self, kind: Follow2dKind) -> Self {
        self.kind = kind;
        self
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

        match follow.kind {
            Follow2dKind::Exponential { seconds } => {
                let delta = follow_exp_diff(
                    tr.translation.xz(),
                    target,
                    seconds,
                    time.delta_seconds(),
                    follow.leeway,
                );
                if delta.length() > 0.01 {
                    follow.diff = delta;
                }
            }

            Follow2dKind::Linear { speed } => {
                follow.diff = (target - tr.translation.xz())
                    .normalize_or_zero()
                    .mul(speed * time.delta_seconds());
            }

            Follow2dKind::Snap => {
                follow.diff = target - tr.translation.xz();
            }
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
