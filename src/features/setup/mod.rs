pub mod trees;
pub mod camera;
pub mod light;
use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(trees::TreesPlugin);
        app.add_plugins(camera::CameraPlugin);
        app.add_plugins(light::LightPlugin);
    }
}

