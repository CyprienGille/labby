use bevy::prelude::*;
use once_cell::sync::Lazy;

use crate::{board::Board, boards_repository::BOARD_0};

#[derive(Resource)]
pub struct SelectedBoard {
    pub board: Board,
}

pub struct BoardSelectorPlugin;

impl Plugin for BoardSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, select_board);
    }
}

fn select_board(mut commands: Commands) {
    commands.insert_resource(SelectedBoard {
        board: Lazy::force(&BOARD_0).clone(),
    })
}
