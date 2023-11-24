use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::f32::consts::PI;

use crate::movement::CanMove;
use crate::GridPosition;

pub const TILE_SCALE: Vec3 = Vec3::new(0.3, 0.3, 0.0);
pub const TILE_SIZE: Vec3 = Vec3::new(900.0, 900.0, 0.0);

#[derive(Debug, Default, PartialEq, Component, Clone, Copy)]
pub enum TileType {
    Corner,
    Straight,
    T,
    FourWay,
    Block,
    #[default]
    Any,
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct OpenWays {
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

#[derive(Bundle, Default)]
pub struct TileBundle {
    tile_type: TileType,
    pos: GridPosition,
    can_move: CanMove,
    sprite: SpriteBundle,
    open_ways: OpenWays,
}

pub fn spawn_tile(
    x_pos: i32,
    y_pos: i32,
    mut tile_type: TileType,
    angle: f32,
    can_move: CanMove,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    // If tile_type is Any, pick one among a list of chosen types
    if tile_type == TileType::Any {
        tile_type = *vec![TileType::Corner, TileType::Straight, TileType::T]
            .choose(&mut thread_rng())
            .unwrap_or(&TileType::Corner);
    }

    let texture_path = match tile_type {
        TileType::Corner => "corner.png",
        TileType::Straight => "straight.png",
        TileType::T => "T_shape.png",
        TileType::FourWay => "4_way.png",
        TileType::Block => "Block.png",
        TileType::Any => "corner.png", //Should never be reached, default to corner
    };

    let open_ways = match tile_type {
        TileType::Corner => OpenWays {
            top: false,
            right: false,
            bottom: true,
            left: true,
        },
        TileType::Straight => OpenWays {
            top: false,
            right: true,
            bottom: false,
            left: true,
        },
        TileType::T => OpenWays {
            top: false,
            right: true,
            bottom: true,
            left: true,
        },
        TileType::FourWay => OpenWays {
            top: true,
            right: true,
            bottom: true,
            left: true,
        },
        TileType::Block => OpenWays {
            top: false,
            right: false,
            bottom: false,
            left: false,
        },
        // Should never be reached, default to corner
        TileType::Any => OpenWays {
            top: false,
            right: false,
            bottom: true,
            left: true,
        },
    };

    commands.spawn(TileBundle {
        pos: GridPosition { x_pos, y_pos },
        tile_type,
        can_move,
        sprite: SpriteBundle {
            texture: asset_server.load(texture_path),
            transform: Transform {
                translation: Vec3::new(
                    x_pos as f32 * TILE_SIZE.x * TILE_SCALE.x,
                    y_pos as f32 * TILE_SIZE.y * TILE_SCALE.y,
                    0.0,
                ),
                scale: TILE_SCALE,
                rotation: Quat::from_axis_angle(
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    angle,
                ),
            },
            ..default()
        },
        open_ways: rotate_ways(open_ways, angle),
    });
}

fn rotate_ways(mut open_ways: OpenWays, angle: f32) -> OpenWays {
    let old_ways = open_ways;
    if angle == PI || angle == -PI {
        // half-turn
        open_ways.top = old_ways.bottom;
        open_ways.bottom = old_ways.top;
        open_ways.right = old_ways.left;
        open_ways.left = old_ways.right;
        open_ways
    } else if angle == PI / 2.0 || angle == -3.0 * PI / 2.0 {
        // quarter turn anti-clockwise
        open_ways.top = old_ways.right;
        open_ways.bottom = old_ways.left;
        open_ways.right = old_ways.bottom;
        open_ways.left = old_ways.top;
        open_ways
    } else if angle == -PI / 2.0 || angle == 3.0 * PI / 2.0 {
        // quarter turn clockwise
        open_ways.top = old_ways.left;
        open_ways.bottom = old_ways.right;
        open_ways.right = old_ways.top;
        open_ways.left = old_ways.bottom;
        open_ways
    } else {
        // println!("No rotation due to angle being {}", angle);
        old_ways
    }
}
