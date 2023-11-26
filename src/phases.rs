use bevy::prelude::*;

use crate::{movement::CanMove, player::Player, GameState, NUM_PLAYERS};

pub struct GamePhasePlugin;

impl Plugin for GamePhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_phase);
    }
}

fn check_phase(
    mut player_query: Query<(&Player, &mut CanMove)>,
    mut game_state: ResMut<GameState>,
    keys: Res<Input<KeyCode>>,
) {
    for (player, mut can_move) in &mut player_query {
        // if it is a player's turn, allow them to move
        if game_state.current_player_id == player.id && !game_state.tile_push_phase {
            *can_move = CanMove::Yes;
        } else {
            *can_move = CanMove::No;
        }
    }
    if keys.just_pressed(KeyCode::T) {
        // end turn
        game_state.current_player_id = (game_state.current_player_id + 1) % NUM_PLAYERS;
        game_state.tile_push_phase = true;
    }
    if keys.just_released(KeyCode::Return) {
        game_state.tile_push_phase = false;
    }
}
