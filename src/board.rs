use bevy::prelude::*;
use ndarray::prelude::*;
use once_cell::sync::Lazy;

use crate::boards_repository::BOARD_0;
use crate::movement::CanMove;
use crate::tile::spawn_tile;
use crate::tile::TileType;

#[derive(Component, Debug, Clone)]
pub struct Board {
    pub tiles: Array2<TileInfo>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TileInfo {
    pub tile_type: TileType,
    pub angle: f32,
    pub can_move: CanMove,
}

pub static NUM_TILES_X: Lazy<usize> = Lazy::new(|| Lazy::force(&BOARD_0).tiles.shape()[1]);
pub static NUM_TILES_Y: Lazy<usize> = Lazy::new(|| Lazy::force(&BOARD_0).tiles.shape()[0]);
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board);
    }
}

fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let board = Lazy::force(&BOARD_0).clone();

    for y_pos in 0..*Lazy::force(&NUM_TILES_Y) {
        for x_pos in 0..*Lazy::force(&NUM_TILES_X) {
            let current_tile = board.tiles[[y_pos, x_pos]];
            // println!("{:?}", current_tile);
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

    commands.spawn(board);
}
