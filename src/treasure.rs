use bevy::{prelude::*, utils::HashMap};
use std::fs::read_dir;

use crate::{
    actors::{get_random_pos_on_axis, GridAxis, SpawnPosition},
    board_selector::SelectedBoard,
    movement::{get_max_coords, pos_is_external},
    tile::{TileType, TILE_SIZE},
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

#[derive(Resource, Debug, Default)]
struct TreasureLists {
    lists: HashMap<i32, Vec<i32>>,
}

pub struct TreasurePlugin;

impl Plugin for TreasurePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TreasureLists { ..default() })
            .add_systems(Startup, spawn_all_treasures)
            .add_systems(PostStartup, init_treasure_lists)
            .add_systems(Update, move_treasure_with_ext_tile);
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

    let num_set_pos: i32 = selected_board
        .board
        .treasure_positions
        .len()
        .try_into()
        .unwrap();

    let mut all_player_spawns: Vec<GridPosition> = vec![];
    let mut used_pos: Vec<GridPosition> = vec![];

    for spawn_pos in &selected_board.board.spawn_positions {
        match spawn_pos {
            SpawnPosition::Position(grid_pos) => all_player_spawns.push(*grid_pos),
            SpawnPosition::Any => (),
        };
    }

    for id in 0..(game_settings.num_players * game_settings.treasures_to_get) {
        let mut current_spawn_pos = SpawnPosition::Any;

        if id < num_set_pos {
            current_spawn_pos = selected_board.board.treasure_positions[id as usize];
        }

        match current_spawn_pos {
            SpawnPosition::Position(GridPosition { x_pos, y_pos }) => {
                spawn_treasure(
                    id,
                    x_pos,
                    y_pos,
                    &mut commands,
                    &asset_server,
                    &sprite_paths,
                );
                used_pos.push(GridPosition { x_pos, y_pos });
            }
            SpawnPosition::Any => {
                let mut x_pos = get_random_pos_on_axis(GridAxis::X, &selected_board);
                let mut y_pos = get_random_pos_on_axis(GridAxis::Y, &selected_board);
                while all_player_spawns.contains(&GridPosition { x_pos, y_pos })
                    || used_pos.contains(&GridPosition { x_pos, y_pos })
                {
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
                used_pos.push(GridPosition { x_pos, y_pos });
            }
        }
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

fn init_treasure_lists(treasure_lists: ResMut<TreasureLists>, game_settings: Res<GameSettings>) {}

fn move_treasure_with_ext_tile(
    tiles_query: Query<&GridPosition, With<TileType>>,
    mut treasures_query: Query<(&mut GridPosition, &mut Transform, &Treasure), Without<TileType>>,
    selected_board: Res<SelectedBoard>,
) {
    let (max_x, max_y) = get_max_coords(&selected_board);

    for tile_grid_pos in &tiles_query {
        if pos_is_external(tile_grid_pos, max_x, max_y) {
            for (mut treasure_grid_pos, mut treasure_transform, _treasure) in &mut treasures_query {
                if pos_is_external(&treasure_grid_pos, max_x, max_y) {
                    *treasure_grid_pos = *tile_grid_pos;
                    treasure_transform.translation = Vec3::new(
                        tile_grid_pos.x_pos as f32 * TILE_SIZE.x * TREASURE_SCALE.x,
                        tile_grid_pos.y_pos as f32 * TILE_SIZE.y * TREASURE_SCALE.y,
                        2.0,
                    )
                }
            }
        }
    }
}
