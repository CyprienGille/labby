use bevy::prelude::*;
use once_cell::sync::Lazy;

use crate::{
    board::{NUM_TILES_X, NUM_TILES_Y},
    tile::{TILE_SCALE, TILE_SIZE},
};

const CAMERA_LEVEL: f32 = 20.0;

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let num_tiles_x = Lazy::force(&NUM_TILES_X);
    let num_tiles_y = Lazy::force(&NUM_TILES_Y);

    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3 {
                x: (num_tiles_x - 1) as f32 * TILE_SCALE.x * TILE_SIZE.x / 2.0,
                y: (num_tiles_y - 1) as f32 * TILE_SCALE.y * TILE_SIZE.y / 2.0,
                z: CAMERA_LEVEL,
            },
            scale: Vec3::new(2.0, 2.0, 1.0),
            ..default()
        },
        ..default()
    });
}
