use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    board_selector::SelectedBoard, movement::CanMove, tile::TILE_SIZE, GameSettings, GridPosition,
};

const TOKEN_SCALE: Vec3 = Vec3::new(0.3, 0.3, 0.0);
// const TOKEN_SIZE: Vec3 = Vec3::new(280.0, 280.0, 0.0);

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

#[derive(Component, Debug)]
pub struct Player {
    pub id: i32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    pos: GridPosition,
    can_move: CanMove,
    sprite: SpriteBundle,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_players);
    }
}

fn spawn_players(
    mut commands: Commands,
    game_settings: Res<GameSettings>,
    selected_board: Res<SelectedBoard>,
    asset_server: Res<AssetServer>,
) {
    let num_spawn_pos = selected_board.board.spawn_positions.len();
    let mut used_pos = vec![];
    for id in 0..game_settings.num_players {
        let mut current_spawn_pos = SpawnPosition::Any;
        if id < num_spawn_pos.try_into().unwrap() {
            current_spawn_pos = selected_board.board.spawn_positions[id as usize]
        }
        match current_spawn_pos {
            SpawnPosition::Position(GridPosition { x_pos, y_pos }) => {
                spawn_player(id, x_pos, y_pos, &mut commands, &asset_server);
                used_pos.push(GridPosition { x_pos, y_pos });
            }
            SpawnPosition::Any => {
                let mut x_pos = get_random_pos_on_axis(GridAxis::X, &selected_board);
                let mut y_pos = get_random_pos_on_axis(GridAxis::Y, &selected_board);
                while used_pos.contains(&GridPosition { x_pos, y_pos })
                    && used_pos.len() < (game_settings.num_players - 1).try_into().unwrap()
                {
                    x_pos = get_random_pos_on_axis(GridAxis::X, &selected_board);
                    y_pos = get_random_pos_on_axis(GridAxis::Y, &selected_board);
                }
                spawn_player(id, x_pos, y_pos, &mut commands, &asset_server);
                used_pos.push(GridPosition { x_pos, y_pos });
            }
        }
    }
}

fn spawn_player(
    id: i32,
    x_pos: i32,
    y_pos: i32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn(PlayerBundle {
        player: Player { id },
        pos: GridPosition { x_pos, y_pos },
        can_move: CanMove::No,
        sprite: SpriteBundle {
            texture: asset_server.load("Commoner.png"),
            transform: Transform {
                translation: Vec3::new(
                    x_pos as f32 * TILE_SIZE.x * TOKEN_SCALE.x,
                    y_pos as f32 * TILE_SIZE.y * TOKEN_SCALE.y,
                    1.0,
                ),
                scale: TOKEN_SCALE,
                ..default()
            },
            ..default()
        },
    });
    // println!(
    //     "Spawned player {:?} at position {:?}",
    //     id,
    //     GridPosition { x_pos, y_pos }
    // );
}

fn get_random_pos_on_axis(axis: GridAxis, selected_board: &Res<SelectedBoard>) -> i32 {
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
