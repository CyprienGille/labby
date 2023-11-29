use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{board_selector::SelectedBoard, GridPosition};

// A spawn position, specified or not
#[derive(Default, Debug, Clone, Copy)]
pub enum SpawnPosition {
    Position(GridPosition),
    #[default]
    Any,
}

#[derive(Debug)]
pub enum GridAxis {
    X,
    Y,
}

pub fn get_random_pos_on_axis(axis: GridAxis, selected_board: &Res<SelectedBoard>) -> i32 {
    // get a random integer in the range of the number of tiles in the specified axis
    let mut rng = thread_rng();

    match axis {
        GridAxis::X => rng
            .gen_range(0..selected_board.board.tiles.shape()[1])
            .try_into()
            .unwrap(),
        GridAxis::Y => rng
            .gen_range(0..selected_board.board.tiles.shape()[0])
            .try_into()
            .unwrap(),
    }
}
