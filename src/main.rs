mod board;
mod board_selector;
mod boards_repository;
mod camera;
mod debug;
mod movement;
mod player;
mod tile;
mod treasure;

use bevy::prelude::*;
use board::BoardPlugin;
use board_selector::BoardSelectorPlugin;
use camera::Camera2dPlugin;
use player::PlayerPlugin;
// use debug::DebugPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.3, 0.2);
const NUM_PLAYERS: i32 = 26;

#[derive(Resource, Debug)]
pub struct GameSettings {
    num_players: i32,
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
        .add_plugins(DefaultPlugins)
        // User plugins
        .add_plugins(BoardSelectorPlugin)
        .add_plugins(Camera2dPlugin)
        .add_plugins(BoardPlugin)
        .add_plugins(PlayerPlugin)
        // Debug plugin
        // .add_plugins(DebugPlugin)
        .run();
}
