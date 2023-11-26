use bevy::prelude::*;
use once_cell::sync::Lazy;

use crate::{
    board::Board,
    boards_repository::{BOARD_0, BOARD_CLASSIC},
};

#[derive(Resource)]
pub struct SelectedBoard {
    pub board: Board,
}

pub struct BoardSelectorPlugin;

impl Plugin for BoardSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, select_board);
    }
}

fn select_board(mut commands: Commands) {
    // Select board on startup from repository
    commands.insert_resource(SelectedBoard {
        board: Lazy::force(&BOARD_CLASSIC).clone(),
    })
}
