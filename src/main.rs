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

use bevy::prelude::*;
use board::BoardPlugin;
use board_selector::BoardSelectorPlugin;
use camera::Camera2dPlugin;
use movement::MovementPlugin;
use phases::GamePhasePlugin;
use player::PlayerPlugin;
// use debug::DebugPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.3, 0.2);
const NUM_PLAYERS: i32 = 4;

#[derive(Resource, Debug)]
pub struct GameSettings {
    num_players: i32,
}

#[derive(Resource, Debug)]
pub struct GameState {
    current_player_id: i32,
    tile_push_phase: bool,
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq)]
pub struct GridPosition {
    x_pos: i32,
    y_pos: i32,
}

fn main() {
    App::new()
        //bevy Built-ins
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(GameSettings {
            num_players: NUM_PLAYERS,
        })
        .insert_resource(GameState {
            current_player_id: 0,
            tile_push_phase: false,
        })
        .add_plugins(DefaultPlugins)
        // User plugins
        .add_plugins(BoardSelectorPlugin)
        .add_plugins(Camera2dPlugin)
        .add_plugins(BoardPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(GamePhasePlugin)
        // Debug plugin
        // .add_plugins(DebugPlugin)
        .run();
}
