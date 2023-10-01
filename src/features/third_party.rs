use bevy::{input::common_conditions::input_toggle_active, prelude::*};
#[cfg(not(target_arch = "wasm32"))]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

pub struct ThirdPartyPlugin;

impl Plugin for ThirdPartyPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::T)));

        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ));
        app.insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO,
            ..default()
        });
    }
}
