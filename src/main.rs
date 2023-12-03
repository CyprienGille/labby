// if not in debug builds, tell windows to not pop up a terminal too
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actors;
mod board;
mod board_selector;
mod boards_repository;
mod camera;
mod debug;
mod movement;
mod phases;
mod player;
mod tile;
mod treasure;
mod ui;

use bevy::prelude::*;

use board::BoardPlugin;
use board_selector::BoardSelectorPlugin;
use camera::Camera2dPlugin;
use movement::MovementPlugin;
use phases::GamePhasePlugin;
use player::PlayerPlugin;
use treasure::TreasurePlugin;
use ui::UIPlugin;
// use debug::DebugPlugin;

// Background color outside of the board
const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.2, 0.1);
// Number of players
const NUM_PLAYERS: i32 = 4;
const TREASURES_TO_GET: i32 = 6;

#[derive(Resource, Debug)]
pub struct GameSettings {
    num_players: i32,
    treasures_to_get: i32,
}

#[derive(Resource, Debug)]
pub struct GameState {
    // The id of the player whose turn it is
    current_player_id: i32,
    // whether we are in the board movement phase
    // (or the player movement phase, if false)
    tile_push_phase: bool,
    // whether the game has ended
    has_ended: bool,
}

// The position of a player, a tile or a treasure in tile units
#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct GridPosition {
    x_pos: i32,
    y_pos: i32,
}

fn main() {
    App::new()
        //Built-ins
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        // Custom Resources
        .insert_resource(GameSettings {
            num_players: NUM_PLAYERS,
            treasures_to_get: TREASURES_TO_GET,
        })
        .insert_resource(GameState {
            current_player_id: 0,
            tile_push_phase: true,
            has_ended: false,
        })
        // Custom plugins
        .add_plugins((
            BoardSelectorPlugin,
            Camera2dPlugin,
            BoardPlugin,
            PlayerPlugin,
            MovementPlugin,
            GamePhasePlugin,
            TreasurePlugin,
            UIPlugin,
        ))
        // Debug plugin
        // .add_plugins(DebugPlugin)
        .run();
}
