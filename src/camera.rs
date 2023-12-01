use bevy::prelude::*;

use crate::{
    board_selector::SelectedBoard,
    tile::{TILE_SCALE, TILE_SIZE},
};

// Used for z-ordering
const CAMERA_LEVEL: f32 = 20.0;

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, zoom_camera);
    }
}

fn spawn_camera(mut commands: Commands, selected_board: Res<SelectedBoard>) {
    let num_tiles_x = selected_board.board.tiles.shape()[1];
    let num_tiles_y = selected_board.board.tiles.shape()[0];

    // spawn camera above middle of the board
    // TODO adjust scale according to number of tiles
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3 {
                x: (num_tiles_x - 1) as f32 * TILE_SCALE.x * TILE_SIZE.x / 2.0,
                y: (num_tiles_y - 1) as f32 * TILE_SCALE.y * TILE_SIZE.y / 2.0,
                z: CAMERA_LEVEL,
            },
            scale: Vec3::new(num_tiles_x as f32 / 2.0, num_tiles_x as f32 / 2.0, 1.0),
            ..default()
        },
        ..default()
    });
}

fn zoom_camera(mut camera_query: Query<&mut Transform, With<Camera2d>>, keys: Res<Input<KeyCode>>) {
    let mut camera_transform = camera_query
        .get_single_mut()
        .expect("More than one Camera2d!");

    if keys.just_pressed(KeyCode::PageUp) {
        camera_transform.scale += Vec3::new(0.2, 0.2, 0.0);
    } else if keys.just_pressed(KeyCode::PageDown) {
        camera_transform.scale -= Vec3::new(0.2, 0.2, 0.0);
    }
}
