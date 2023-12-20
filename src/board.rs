use bevy::prelude::*;
use ndarray::prelude::*;

use crate::actors::SpawnPosition;
use crate::board_selector::SelectedBoard;
use crate::movement::CanMove;
use crate::phases::GameState;
use crate::tile::spawn_tile;
use crate::tile::TileType;

const GROUND_SIZE: Vec3 = Vec3::new(1600.0, 1600.0, 0.0);

/// The general internal board representation
#[derive(Debug, Clone)]
pub struct Board {
    pub tiles: Array2<TileInfo>,
    pub spawn_positions: Vec<SpawnPosition>,
    pub treasure_positions: Vec<SpawnPosition>,
    pub external_tile: TileInfo,
}

// All of the info needed to spawn a tile
#[derive(Debug, Default, Clone, Copy)]
pub struct TileInfo {
    pub tile_type: TileType,
    pub angle: f32,
    pub can_move: CanMove,
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground)
            .add_systems(OnEnter(GameState::Playing), spawn_board)
            .add_systems(OnExit(GameState::Playing), cleanup_board);
    }
}

fn spawn_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selected_board: Res<SelectedBoard>,
) {
    let board = &selected_board.board;
    let num_tiles_y = board.tiles.shape()[0];
    let num_tiles_x = board.tiles.shape()[1];

    for y_pos in 0..num_tiles_y {
        for x_pos in 0..num_tiles_x {
            let current_tile = board.tiles[[y_pos, x_pos]];
            // we need to invert the y index
            // to spawn tiles top-down from the selected board array
            // Note: This panics if the board is immense
            let int_y_pos: i32 = (board.tiles.shape()[0] - y_pos - 1).try_into().unwrap();
            let int_x_pos: i32 = x_pos.try_into().unwrap();
            spawn_tile(
                int_x_pos,
                int_y_pos,
                current_tile.tile_type,
                current_tile.angle,
                current_tile.can_move,
                &mut commands,
                &asset_server,
            );
        }
    }

    // External tile for pushing
    spawn_tile(
        -1,
        0,
        board.external_tile.tile_type,
        board.external_tile.angle,
        board.external_tile.can_move,
        &mut commands,
        &asset_server,
    );
}

fn cleanup_board(mut commands: Commands, tiles_query: Query<Entity, With<TileType>>) {
    for entity in &tiles_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
    for x in -30..31 {
        for y in -30..31 {
            commands.spawn(SpriteBundle {
                texture: asset_server.load("ground.png"),
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * GROUND_SIZE.x,
                        y as f32 * GROUND_SIZE.y,
                        -1.0,
                    ),
                    ..default()
                },
                ..default()
            });
        }
    }
}
