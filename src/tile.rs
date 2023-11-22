use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::movement::CanMove;
use crate::GridPosition;

pub const TILE_SCALE: Vec3 = Vec3::new(0.3, 0.3, 0.0);
pub const TILE_SIZE: Vec3 = Vec3::new(900.0, 900.0, 0.0);

#[derive(Debug, Default, PartialEq, Component, Clone, Copy)]
pub enum TileType {
    Corner,
    Straight,
    T,
    #[default]
    Any,
}

#[derive(Bundle, Default)]
pub struct TileBundle {
    tile_type: TileType,
    pos: GridPosition,
    can_move: CanMove,
    sprite: SpriteBundle,
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
    // If tile_type is Any, pick one of the other tile types
    if tile_type == TileType::Any {
        tile_type = *vec![TileType::Corner, TileType::Straight, TileType::T]
            .choose(&mut thread_rng())
            .unwrap_or(&TileType::Corner);
    }

    let texture_path = match tile_type {
        TileType::Corner => "corner.png",
        TileType::Straight => "straight.png",
        TileType::T => "T_shape.png",
        TileType::Any => "corner.png",
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
    });
}
