use bevy::prelude::*;
use features::FeaturesPlugin;

mod features;
mod macros;
mod util;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_game".into(),
                resolution: (600., 570.).into(),
                // position: WindowPosition::At((010, 30).into()),
                position: WindowPosition::At((1010, 30).into()),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FeaturesPlugin)
        .run();
}
