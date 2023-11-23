use bevy::prelude::*;

use crate::{
    player::Player,
    tile::{TILE_SCALE, TILE_SIZE},
    GameState, GridPosition,
};

#[derive(Debug, Default, Component, Clone, Copy)]
pub enum CanMove {
    #[default]
    Yes,
    No,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_current_player);
    }
}

fn move_current_player(
    mut query: Query<(&mut GridPosition, &mut Transform, &CanMove, &Player)>,
    game_state: Res<GameState>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut grid_pos, mut transform, can_move, player) in &mut query {
        if matches!(can_move, CanMove::Yes) && (player.id == game_state.current_player_id) {
            if keys.just_pressed(KeyCode::Right) {
                grid_pos.x_pos += 1;
                transform.translation.x += TILE_SIZE.x * TILE_SCALE.x
            } else if keys.just_pressed(KeyCode::Left) {
                grid_pos.x_pos -= 1;
                transform.translation.x -= TILE_SIZE.x * TILE_SCALE.x
            } else if keys.just_pressed(KeyCode::Up) {
                grid_pos.y_pos += 1;
                transform.translation.y += TILE_SIZE.y * TILE_SCALE.y
            } else if keys.just_pressed(KeyCode::Down) {
                grid_pos.y_pos -= 1;
                transform.translation.y -= TILE_SIZE.y * TILE_SCALE.y
            }
        }
    }
}
