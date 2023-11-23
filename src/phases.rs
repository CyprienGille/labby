use bevy::prelude::*;

use crate::{movement::CanMove, player::Player, GameState, NUM_PLAYERS};

pub struct GamePhasePlugin;

impl Plugin for GamePhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_phase);
    }
}

fn check_phase(
    mut query: Query<(&Player, &mut CanMove)>,
    mut game_state: ResMut<GameState>,
    keys: Res<Input<KeyCode>>,
) {
    for (player, mut can_move) in &mut query {
        if game_state.current_player_id == player.id {
            *can_move = CanMove::Yes;
        } else {
            *can_move = CanMove::No;
        }
    }
    if keys.just_pressed(KeyCode::T) {
        game_state.current_player_id = (game_state.current_player_id + 1) % NUM_PLAYERS;
    }
}
