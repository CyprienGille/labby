use bevy::prelude::*;

use crate::{
    player::Player,
    tile::{OpenWays, TILE_SCALE, TILE_SIZE},
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
        app.add_systems(Update, move_current_player);
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
                && move_ok(*grid_pos, Direction::Right, &tiles_query)
            {
                // if right arrow was pressed and this movement is legal, move the player
                grid_pos.x_pos += 1;
                transform.translation.x += TILE_SIZE.x * TILE_SCALE.x
            } else if keys.just_pressed(KeyCode::Left)
                && move_ok(*grid_pos, Direction::Left, &tiles_query)
            {
                grid_pos.x_pos -= 1;
                transform.translation.x -= TILE_SIZE.x * TILE_SCALE.x
            } else if keys.just_pressed(KeyCode::Up)
                && move_ok(*grid_pos, Direction::Up, &tiles_query)
            {
                grid_pos.y_pos += 1;
                transform.translation.y += TILE_SIZE.y * TILE_SCALE.y
            } else if keys.just_pressed(KeyCode::Down)
                && move_ok(*grid_pos, Direction::Down, &tiles_query)
            {
                grid_pos.y_pos -= 1;
                transform.translation.y -= TILE_SIZE.y * TILE_SCALE.y
            }
        }
    }
}

fn move_ok(
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
