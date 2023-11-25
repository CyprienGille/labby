use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    board_selector::SelectedBoard,
    player::Player,
    tile::{rotate_ways, OpenWays, TileType, TILE_SCALE, TILE_SIZE},
    GameState, GridPosition,
};

#[derive(Debug, Default, Component, Clone, Copy)]
pub enum CanMove {
    #[default]
    Yes,
    No,
}

// The allowed movement directions
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_current_player)
            .add_systems(Update, move_current_tile);
    }
}

fn move_current_player(
    mut player_query: Query<(&mut GridPosition, &mut Transform, &CanMove, &Player)>,
    tiles_query: Query<(&OpenWays, &GridPosition), Without<Player>>,
    game_state: Res<GameState>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut grid_pos, mut transform, can_move, player) in &mut player_query {
        if matches!(can_move, CanMove::Yes)
            && (player.id == game_state.current_player_id)
            && (!game_state.tile_push_phase)
        {
            // If this player can move, it is their turn and they're not pushing tiles
            if keys.just_pressed(KeyCode::Right)
                && player_move_ok(*grid_pos, Direction::Right, &tiles_query)
            {
                // if right arrow was pressed and this movement is legal, move the player
                grid_pos.x_pos += 1;
                transform.translation.x += TILE_SIZE.x * TILE_SCALE.x
            } else if keys.just_pressed(KeyCode::Left)
                && player_move_ok(*grid_pos, Direction::Left, &tiles_query)
            {
                grid_pos.x_pos -= 1;
                transform.translation.x -= TILE_SIZE.x * TILE_SCALE.x
            } else if keys.just_pressed(KeyCode::Up)
                && player_move_ok(*grid_pos, Direction::Up, &tiles_query)
            {
                grid_pos.y_pos += 1;
                transform.translation.y += TILE_SIZE.y * TILE_SCALE.y
            } else if keys.just_pressed(KeyCode::Down)
                && player_move_ok(*grid_pos, Direction::Down, &tiles_query)
            {
                grid_pos.y_pos -= 1;
                transform.translation.y -= TILE_SIZE.y * TILE_SCALE.y
            }
        }
    }
}

fn player_move_ok(
    prev_pos: GridPosition,
    wanted_dir: Direction,
    tiles_query: &Query<(&OpenWays, &GridPosition), Without<Player>>,
) -> bool {
    // Check if a desired move is legal (no walls, no outside board)

    // The desired position after moving
    let mut destination = GridPosition {
        ..Default::default()
    };
    match wanted_dir {
        Direction::Up => {
            destination.y_pos = prev_pos.y_pos + 1;
            destination.x_pos = prev_pos.x_pos;
        }
        Direction::Down => {
            destination.y_pos = prev_pos.y_pos - 1;
            destination.x_pos = prev_pos.x_pos;
        }
        Direction::Left => {
            destination.x_pos = prev_pos.x_pos - 1;
            destination.y_pos = prev_pos.y_pos;
        }
        Direction::Right => {
            destination.x_pos = prev_pos.x_pos + 1;
            destination.y_pos = prev_pos.y_pos;
        }
    }
    // Openings of the current tile
    let mut current_ways = OpenWays { ..default() };
    // Openings of the destination tile
    let mut dest_ways = OpenWays { ..default() };

    for (open_ways, grid_pos) in tiles_query {
        // get the openings
        if grid_pos == &prev_pos {
            current_ways = *open_ways;
        }
        if grid_pos == &destination {
            dest_ways = *open_ways;
        }
    }
    match wanted_dir {
        // evaluates to true if both tiles are open for the desired move
        Direction::Up => current_ways.top && dest_ways.bottom,
        Direction::Down => current_ways.bottom && dest_ways.top,
        Direction::Left => current_ways.left && dest_ways.right,
        Direction::Right => current_ways.right && dest_ways.left,
    }
}

fn move_current_tile(
    mut tiles_query: Query<(&mut GridPosition, &mut Transform, &mut OpenWays), With<TileType>>,
    game_state: Res<GameState>,
    selected_board: Res<SelectedBoard>,
    keys: Res<Input<KeyCode>>,
) {
    let max_x: i32 = (selected_board.board.tiles.shape()[1] - 1)
        .try_into()
        .unwrap();
    let max_y: i32 = (selected_board.board.tiles.shape()[0] - 1)
        .try_into()
        .unwrap();
    if game_state.tile_push_phase {
        for (mut grid_pos, mut transform, mut open_ways) in &mut tiles_query {
            if grid_pos.x_pos == -1
                || grid_pos.y_pos == -1
                || grid_pos.x_pos > max_x
                || grid_pos.y_pos > max_y
            {
                // This is the external tile
                if keys.just_pressed(KeyCode::Up)
                    && tile_move_ok(&grid_pos, Direction::Up, max_x, max_y)
                {
                    if grid_pos.y_pos == -1 {
                        // Move to the other side of the board
                        grid_pos.y_pos = max_y + 1;
                        transform.translation.y = (max_y + 1) as f32 * TILE_SIZE.y * TILE_SCALE.y;
                    } else if grid_pos.y_pos < max_y {
                        // move up along the board
                        grid_pos.y_pos += 1;
                        transform.translation.y += TILE_SIZE.y * TILE_SCALE.y;
                    } else if grid_pos.y_pos == max_y {
                        // move to the top side of the board
                        grid_pos.y_pos += 1;
                        if grid_pos.x_pos == -1 {
                            // from the left side of the board
                            grid_pos.x_pos += 1;
                            transform.translation += Vec3::new(
                                TILE_SIZE.x * TILE_SCALE.x,
                                TILE_SIZE.y * TILE_SCALE.y,
                                0.0,
                            );
                        }
                        if grid_pos.x_pos == max_x + 1 {
                            // from the right side of the board
                            grid_pos.x_pos -= 1;

                            transform.translation += Vec3::new(
                                -TILE_SIZE.x * TILE_SCALE.x,
                                TILE_SIZE.y * TILE_SCALE.y,
                                0.0,
                            );
                        }
                    }
                } else if keys.just_pressed(KeyCode::Down)
                    && tile_move_ok(&grid_pos, Direction::Down, max_x, max_y)
                {
                    if grid_pos.y_pos == max_y + 1 {
                        // Move to the other side of the board
                        grid_pos.y_pos = -1;
                        transform.translation.y = -TILE_SIZE.y * TILE_SCALE.y;
                    } else if grid_pos.y_pos > 0 {
                        // move down along the board
                        grid_pos.y_pos -= 1;
                        transform.translation.y -= TILE_SIZE.y * TILE_SCALE.y;
                    } else if grid_pos.y_pos == 0 {
                        // move to the bottom side of the board
                        grid_pos.y_pos -= 1;
                        if grid_pos.x_pos == -1 {
                            // from the left side of the board
                            grid_pos.x_pos += 1;
                            transform.translation += Vec3::new(
                                TILE_SIZE.x * TILE_SCALE.x,
                                -TILE_SIZE.y * TILE_SCALE.y,
                                0.0,
                            );
                        }
                        if grid_pos.x_pos == max_x + 1 {
                            // from the right side of the board
                            grid_pos.x_pos -= 1;
                            transform.translation += Vec3::new(
                                -TILE_SIZE.x * TILE_SCALE.x,
                                -TILE_SIZE.y * TILE_SCALE.y,
                                0.0,
                            );
                        }
                    }
                } else if keys.just_pressed(KeyCode::Right)
                    && tile_move_ok(&grid_pos, Direction::Right, max_x, max_y)
                {
                    if grid_pos.x_pos == -1 {
                        // Move to the other side of the board
                        grid_pos.x_pos = max_x + 1;
                        transform.translation.x = (max_x + 1) as f32 * TILE_SIZE.x * TILE_SCALE.x;
                    } else if grid_pos.x_pos < max_x {
                        // move right along the board
                        grid_pos.x_pos += 1;
                        transform.translation.x += TILE_SIZE.x * TILE_SCALE.x;
                    } else if grid_pos.x_pos == max_x {
                        // move to the right side of the board
                        grid_pos.x_pos += 1;
                        if grid_pos.y_pos == -1 {
                            // from the bottom side of the board
                            grid_pos.y_pos += 1;
                            transform.translation += Vec3::new(
                                TILE_SIZE.x * TILE_SCALE.x,
                                TILE_SIZE.y * TILE_SCALE.y,
                                0.0,
                            );
                        }
                        if grid_pos.y_pos == max_y + 1 {
                            // from the top side of the board
                            grid_pos.y_pos -= 1;
                            transform.translation += Vec3::new(
                                TILE_SIZE.x * TILE_SCALE.x,
                                -TILE_SIZE.y * TILE_SCALE.y,
                                0.0,
                            );
                        }
                    }
                } else if keys.just_pressed(KeyCode::Left)
                    && tile_move_ok(&grid_pos, Direction::Left, max_x, max_y)
                {
                    if grid_pos.x_pos == max_x + 1 {
                        // Move to the other side of the board
                        grid_pos.x_pos = -1;
                        transform.translation.x = -TILE_SIZE.x * TILE_SCALE.x;
                    } else if grid_pos.x_pos > 0 {
                        // move left along the board
                        grid_pos.x_pos -= 1;
                        transform.translation.x -= TILE_SIZE.x * TILE_SCALE.x;
                    } else if grid_pos.x_pos == 0 {
                        // move to the left side of the board
                        grid_pos.x_pos -= 1;
                        if grid_pos.y_pos == -1 {
                            // from the bottom side of the board
                            grid_pos.y_pos += 1;
                            transform.translation += Vec3::new(
                                -TILE_SIZE.x * TILE_SCALE.x,
                                TILE_SIZE.y * TILE_SCALE.y,
                                0.0,
                            );
                        }
                        if grid_pos.y_pos == max_y + 1 {
                            // from the top side of the board
                            grid_pos.y_pos -= 1;
                            transform.translation += Vec3::new(
                                -TILE_SIZE.x * TILE_SCALE.x,
                                -TILE_SIZE.y * TILE_SCALE.y,
                                0.0,
                            );
                        }
                    }
                }
                if keys.just_pressed(KeyCode::R) {
                    *open_ways = rotate_ways(*open_ways, -PI / 2.0);
                    transform.rotate_axis(Vec3::new(0.0, 0.0, 1.0), -PI / 2.0);
                }
            }
        }
    }
}

fn tile_move_ok(grid_pos: &GridPosition, wanted_dir: Direction, max_x: i32, max_y: i32) -> bool {
    match wanted_dir {
        Direction::Up => grid_pos.y_pos < max_y + 1,
        Direction::Down => grid_pos.y_pos > -1,
        Direction::Left => grid_pos.x_pos > -1,
        Direction::Right => grid_pos.x_pos < max_x + 1,
    }
}
