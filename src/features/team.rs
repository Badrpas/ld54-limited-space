use bevy::prelude::*;

pub struct TeamPlugin;

impl Plugin for TeamPlugin {
    fn build(&self, _app: &mut App) {
    }
}

#[derive(Component)]
pub enum Team {
    Player,
    Enemy,
}


