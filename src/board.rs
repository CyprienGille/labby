use bevy::prelude::*;
use ndarray::prelude::*;

use crate::board_selector::SelectedBoard;
use crate::movement::CanMove;
use crate::player::SpawnPosition;
use crate::tile::spawn_tile;
use crate::tile::TileType;

// The general internal board representation
#[derive(Debug, Clone)]
pub struct Board {
    pub tiles: Array2<TileInfo>,
    pub spawn_positions: Vec<SpawnPosition>,
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
        app.add_systems(PostStartup, spawn_board);
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
            // To spawn tiles top-down from the selected board array
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
}
