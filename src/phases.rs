use bevy::prelude::*;

use crate::{
    movement::CanMove, player::Player, treasure::CollectedLists, GameState, NUM_PLAYERS,
    TREASURES_TO_GET,
};

pub struct GamePhasePlugin;

impl Plugin for GamePhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_phase);
    }
}

fn check_phase(
    mut player_query: Query<(&Player, &mut CanMove)>,
    mut game_state: ResMut<GameState>,
    collected_treasures: Res<CollectedLists>,
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
        end_turn(&mut game_state, &collected_treasures)
    }
}

pub fn end_turn(game_state: &mut GameState, collected_treasures: &CollectedLists) {
    // end turn
    game_state.tile_push_phase = true;
    game_state.current_player_id = (game_state.current_player_id + 1) % NUM_PLAYERS;
    let mut num_players_finished = 0;
    while (TREASURES_TO_GET as usize
        == collected_treasures
            .lists
            .get(&game_state.current_player_id)
            .unwrap()
            .len())
        && !game_state.has_ended
    {
        game_state.current_player_id = (game_state.current_player_id + 1) % NUM_PLAYERS;
        num_players_finished += 1;
        game_state.has_ended = num_players_finished == NUM_PLAYERS;
    }
}
