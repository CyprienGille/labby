use bevy::prelude::*;
use std::fs::read_dir;

use crate::{
    actors::{get_random_pos_on_axis, GridAxis, SpawnPosition},
    board_selector::SelectedBoard,
    tile::TILE_SIZE,
    GameSettings, GridPosition,
};

const TREASURE_SCALE: Vec3 = Vec3::new(0.3, 0.3, 0.0);

#[derive(Debug, Component)]
struct Treasure {
    id: i32,
}

#[derive(Bundle)]
struct TreasureBundle {
    treasure: Treasure,
    pos: GridPosition,
    sprite: SpriteBundle,
}

pub struct TreasurePlugin;

impl Plugin for TreasurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_all_treasures);
    }
}

fn spawn_all_treasures(
    mut commands: Commands,
    game_settings: Res<GameSettings>,
    selected_board: Res<SelectedBoard>,
    asset_server: Res<AssetServer>,
) {
    let sprite_paths = read_dir("assets/treasures/")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().into_owned())
        .map(|s| s.replace("assets/", ""))
        .collect::<Vec<_>>();

    let mut all_player_spawns: Vec<GridPosition> = vec![];

    for spawn_pos in &selected_board.board.spawn_positions {
        match spawn_pos {
            SpawnPosition::Position(grid_pos) => all_player_spawns.push(*grid_pos),
            SpawnPosition::Any => (),
        };
    }

    for id in 0..(game_settings.num_players * game_settings.treasures_to_get) {
        let mut x_pos = get_random_pos_on_axis(GridAxis::X, &selected_board);
        let mut y_pos = get_random_pos_on_axis(GridAxis::Y, &selected_board);
        while all_player_spawns.contains(&GridPosition { x_pos, y_pos }) {
            x_pos = get_random_pos_on_axis(GridAxis::X, &selected_board);
            y_pos = get_random_pos_on_axis(GridAxis::Y, &selected_board);
        }
        spawn_treasure(
            id,
            x_pos,
            y_pos,
            &mut commands,
            &asset_server,
            &sprite_paths,
        );
    }
}

fn spawn_treasure(
    id: i32,
    x_pos: i32,
    y_pos: i32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    sprite_paths: &Vec<String>,
) {
    commands.spawn(TreasureBundle {
        treasure: Treasure { id },
        pos: GridPosition { x_pos, y_pos },
        sprite: SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    x_pos as f32 * TILE_SIZE.x * TREASURE_SCALE.x,
                    y_pos as f32 * TILE_SIZE.y * TREASURE_SCALE.y,
                    2.0,
                ),
                scale: TREASURE_SCALE,
                ..default()
            },
            texture: asset_server.load(&sprite_paths[id as usize % sprite_paths.len()]),
            ..default()
        },
    });
}
