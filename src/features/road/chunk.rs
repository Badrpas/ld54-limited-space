use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{features::player::PlayerRoot, macros::*};

pub struct RoadChunkPlugin;

impl Plugin for RoadChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Road>();
        app.add_systems(Update, chunk_system);
    }
}

#[derive(Component)]
pub struct RoadChunk;

pub const CHUNK_SIZE: f32 = 18.;

#[derive(Resource, Default)]
pub struct Road {
    pub end: f32,
    pub start: f32,
    pub chunks: VecDeque<Entity>,
}

const ROAD_WINDOW_FW: f32 = CHUNK_SIZE * 2.;
const ROAD_WINDOW_BK: f32 = CHUNK_SIZE * 4.;


pub fn chunk_system(
    mut commands: Commands,
    mut road: ResMut<Road>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player: Query<&Transform, With<PlayerRoot>>,
) {
    let player = ok_or_ret!(player.get_single());
    let player_pos = -player.translation.z;
    while player_pos + ROAD_WINDOW_FW > road.end {
        let chunk = commands.spawn((
            RoadChunk,
            Name::new("Road chunk"),
            PbrBundle {
                mesh: meshes.add(shape::Plane::from_size(CHUNK_SIZE).into()),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform::from_xyz(
                    0.,
                    0.,
                    -road.end,
                ),
                ..default()
            },
        )).id();
        log::info!("Added chunk at z={:.2}", -road.end);
        road.end += CHUNK_SIZE;
        road.chunks.push_back(chunk);
    }

    while player_pos - ROAD_WINDOW_BK > road.start {
        log::info!("Deleting chunk at {:.2}", road.start);
        if let Some(e) = road.chunks.pop_front() {
            commands.entity(e).despawn();
        }
        road.start += CHUNK_SIZE;
    }
}
