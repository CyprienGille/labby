mod board;
mod boards_repository;
mod camera;
mod debug;
mod movement;
mod player;
mod tile;
mod treasure;

use bevy::prelude::*;
use board::BoardPlugin;
use camera::Camera2dPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.8, 0.9, 0.8);

#[derive(Component, Debug, Default)]
pub struct GridPosition {
    x_pos: i32,
    y_pos: i32,
}

fn main() {
    App::new()
        //bevy Built-ins
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        // User plugins
        .add_plugins(Camera2dPlugin)
        .add_plugins(BoardPlugin)
        .run();
}
