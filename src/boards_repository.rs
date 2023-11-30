use ndarray::prelude::*;
use once_cell::sync::Lazy;
use std::f32::consts::PI;

use crate::actors::SpawnPosition;
use crate::board::Board;
use crate::board::TileInfo;
use crate::movement::CanMove;
use crate::tile::TileType;
use crate::GridPosition;

// Set board with set tiles and Any tiles, and spawn positions
pub static BOARD_0: Lazy<Board> = Lazy::new(|| Board {
    tiles: array![
        [
            TileInfo {
                tile_type: TileType::Corner,
                angle: PI / 2.0,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: 0.0,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::Corner,
                angle: 0.0,
                can_move: CanMove::No,
            },
        ],
        [
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
        ],
        [
            TileInfo {
                tile_type: TileType::T,
                angle: PI / 2.0,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: -PI / 2.0,
                can_move: CanMove::No,
            },
        ],
        [
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
        ],
        [
            TileInfo {
                tile_type: TileType::Corner,
                angle: PI,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: PI,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::Corner,
                angle: -PI / 2.0,
                can_move: CanMove::No,
            },
        ],
    ],
    spawn_positions: vec![
        SpawnPosition::Position(GridPosition { x_pos: 0, y_pos: 0 }),
        SpawnPosition::Position(GridPosition { x_pos: 0, y_pos: 4 }),
        SpawnPosition::Position(GridPosition { x_pos: 4, y_pos: 4 }),
        SpawnPosition::Position(GridPosition { x_pos: 4, y_pos: 0 }),
    ],
    treasure_positions: vec![
        SpawnPosition::Any,
        SpawnPosition::Any,
        SpawnPosition::Any,
        SpawnPosition::Any,
    ],
    external_tile: TileInfo {
        tile_type: TileType::Corner,
        angle: 0.0,
        ..Default::default()
    },
});

pub static BOARD_CLASSIC: Lazy<Board> = Lazy::new(|| Board {
    tiles: array![
        [
            TileInfo {
                tile_type: TileType::Corner,
                angle: PI / 2.0,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: 0.0,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: 0.0,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::Corner,
                angle: 0.0,
                can_move: CanMove::No,
            },
        ],
        [
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
        ],
        [
            TileInfo {
                tile_type: TileType::T,
                angle: PI / 2.0,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: PI / 2.0,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: 0.0,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: -PI / 2.0,
                can_move: CanMove::No,
            },
        ],
        [
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
        ],
        [
            TileInfo {
                tile_type: TileType::T,
                angle: PI / 2.0,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: PI,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: -PI / 2.0,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: -PI / 2.0,
                can_move: CanMove::No,
            },
        ],
        [
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                ..Default::default()
            },
        ],
        [
            TileInfo {
                tile_type: TileType::Corner,
                angle: PI,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: PI,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::T,
                angle: PI,
                can_move: CanMove::No,
            },
            TileInfo {
                ..Default::default()
            },
            TileInfo {
                tile_type: TileType::Corner,
                angle: -PI / 2.0,
                can_move: CanMove::No,
            },
        ],
    ],
    spawn_positions: vec![
        SpawnPosition::Position(GridPosition { x_pos: 0, y_pos: 0 }),
        SpawnPosition::Position(GridPosition { x_pos: 0, y_pos: 6 }),
        SpawnPosition::Position(GridPosition { x_pos: 6, y_pos: 6 }),
        SpawnPosition::Position(GridPosition { x_pos: 6, y_pos: 0 }),
    ],
    treasure_positions: vec![
        SpawnPosition::Any,
        SpawnPosition::Any,
        SpawnPosition::Any,
        SpawnPosition::Any,
    ],
    external_tile: TileInfo {
        tile_type: TileType::Corner,
        angle: 0.0,
        ..Default::default()
    },
});
