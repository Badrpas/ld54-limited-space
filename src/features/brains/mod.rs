pub mod slug;
use bevy::prelude::*;

pub struct BrainsPlugin;

impl Plugin for BrainsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(slug::SlugPlugin);
    }
}

