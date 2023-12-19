use bevy::prelude::*;

use crate::{
    movement::CanMove, player::Player, treasure::CollectedLists, GamePhase, NUM_PLAYERS,
    TREASURES_TO_GET,
};

#[derive(Debug, States, PartialEq, Eq, Hash, Clone, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
}

pub struct GamePhasePlugin;

impl Plugin for GamePhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(Update, start_playing.run_if(in_state(GameState::MainMenu)))
            .add_systems(
                Update,
                (check_phase, stop_playing).run_if(in_state(GameState::Playing)),
            );
    }
}

fn start_playing(keys: Res<Input<KeyCode>>, mut game_phase: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Return) {
        game_phase.set(GameState::Playing);
    }
}
fn stop_playing(keys: Res<Input<KeyCode>>, mut game_phase: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        game_phase.set(GameState::MainMenu);
    }
}

fn check_phase(
    mut player_query: Query<(&Player, &mut CanMove)>,
    mut game_phase: ResMut<GamePhase>,
    collected_treasures: Res<CollectedLists>,
    keys: Res<Input<KeyCode>>,
) {
    for (player, mut can_move) in &mut player_query {
        // if it is a player's turn, allow them to move
        if game_phase.current_player_id == player.id && !game_phase.tile_push_phase {
            *can_move = CanMove::Yes;
        } else {
            *can_move = CanMove::No;
        }
    }
    if keys.just_pressed(KeyCode::T) {
        end_turn(&mut game_phase, &collected_treasures)
    }
}

pub fn end_turn(game_state: &mut GamePhase, collected_treasures: &CollectedLists) {
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
