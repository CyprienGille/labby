use ndarray::prelude::*;
use once_cell::sync::Lazy;
use std::f32::consts::PI;

use crate::board::Board;
use crate::board::TileInfo;
use crate::movement::CanMove;
use crate::tile::TileType;

pub static BOARD_0: Lazy<Board> = Lazy::new(|| Board {
    tiles: array![
        [
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::Straight,
                angle: PI / 2.0,
                can_move: CanMove::Yes
            }
        ],
        [
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::Straight,
                angle: 0.0,
                can_move: CanMove::Yes
            }
        ],
        [
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::Corner,
                angle: 3.0 * PI / 2.0,
                can_move: CanMove::Yes
            }
        ],
    ],
});
