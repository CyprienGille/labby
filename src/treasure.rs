use bevy::{prelude::*, utils::HashMap};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::read_dir;

use crate::{
    actors::{get_random_pos_on_axis, GridAxis, SpawnPosition},
    board_selector::SelectedBoard,
    movement::{get_max_coords, pos_is_external},
    phases::{end_turn, GamePhase},
    player::Player,
    tile::{TileType, TILE_SCALE, TILE_SIZE},
    GameSettings, GameState, GridPosition,
};

const TREASURE_SCALE: Vec3 = Vec3::new(0.28, 0.28, 0.0);
const BIG_TREASURE_SCALE: Vec3 = Vec3::new(0.9, 0.9, 0.0);

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
    // the to-be-collected treasure list for each player
    lists: HashMap<i32, Vec<i32>>,
}

#[derive(Resource, Debug, Default)]
pub struct CollectedLists {
    // the collected treasure list for each player
    pub lists: HashMap<i32, Vec<i32>>,
}

#[derive(Default, Debug, Resource)]
struct TreasureSprites {
    list: Vec<String>,
}

pub struct TreasurePlugin;

impl Plugin for TreasurePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TreasureLists { ..default() })
            .insert_resource(CollectedLists { ..default() })
            .insert_resource(TreasureSprites { ..default() })
            .add_systems(
                OnEnter(GamePhase::Playing),
                (spawn_all_treasures, init_treasure_lists),
            )
            .add_systems(
                Update,
                (
                    move_treasure_with_ext_tile,
                    collect_treasure,
                    display_current_treasure,
                )
                    .run_if(in_state(GamePhase::Playing)),
            )
            .add_systems(OnExit(GamePhase::Playing), (cleanup_treasures, reset_lists));
    }
}

fn spawn_all_treasures(
    mut commands: Commands,
    game_settings: Res<GameSettings>,
    selected_board: Res<SelectedBoard>,
    mut treasure_sprites: ResMut<TreasureSprites>,
    asset_server: Res<AssetServer>,
) {
    // all of the available treasure sprites
    let sprite_paths = read_dir("assets/treasures/")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().into_owned())
        .map(|s| s.replace("assets/", ""))
        .collect::<Vec<_>>();

    treasure_sprites.list = sprite_paths.clone();

    // the number of set treasure positions in the selected board
    let num_set_pos: i32 = selected_board
        .board
        .treasure_positions
        .len()
        .try_into()
        .unwrap();

    let mut all_player_spawns: Vec<GridPosition> = vec![];
    let mut used_pos: Vec<GridPosition> = vec![];

    // collect set player spawns to avoid spawning treasures on top of them
    for spawn_pos in &selected_board.board.spawn_positions {
        match spawn_pos {
            SpawnPosition::Position(grid_pos) => all_player_spawns.push(*grid_pos),
            SpawnPosition::Any => (),
        };
    }

    for id in 0..(game_settings.num_players * game_settings.treasures_to_get) {
        let mut current_spawn_pos = SpawnPosition::Any;

        // if there are set positions left
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
                    &sprite_paths[id as usize % sprite_paths.len()],
                    false,
                );
                used_pos.push(GridPosition { x_pos, y_pos });
            }
            SpawnPosition::Any => {
                // get random unused position
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
                    &sprite_paths[id as usize % sprite_paths.len()],
                    false,
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
    sprite_path: &String,
    use_big_scale: bool,
) {
    commands.spawn(TreasureBundle {
        treasure: Treasure { id },
        pos: GridPosition { x_pos, y_pos },
        sprite: SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    x_pos as f32 * TILE_SIZE.x * TILE_SCALE.x,
                    y_pos as f32 * TILE_SIZE.y * TILE_SCALE.y,
                    2.0,
                ),
                scale: if use_big_scale {
                    BIG_TREASURE_SCALE
                } else {
                    TREASURE_SCALE
                },
                ..default()
            },
            texture: asset_server.load(sprite_path),
            ..default()
        },
    });
}

fn init_treasure_lists(
    mut treasure_lists: ResMut<TreasureLists>,
    mut collected_lists: ResMut<CollectedLists>,
    game_settings: Res<GameSettings>,
) {
    // Init the to-be-collected treasure lists

    let mut rng = thread_rng();

    // shuffled list of all of the valid treasure ids
    let mut all_treasure_ids =
        (0..game_settings.num_players * game_settings.treasures_to_get).collect::<Vec<i32>>();
    all_treasure_ids.shuffle(&mut rng);

    for (player_id, chunk) in all_treasure_ids
        .chunks(game_settings.treasures_to_get.try_into().unwrap())
        .enumerate()
    {
        // iterate on chunks of size treasures_to_get of the shuffled list of ids
        treasure_lists
            .lists
            .insert(player_id.try_into().unwrap(), chunk.to_vec());

        // Init the collected treasures list for this player
        collected_lists
            .lists
            .insert(player_id.try_into().unwrap(), vec![]);
    }
}

fn move_treasure_with_ext_tile(
    tiles_query: Query<&GridPosition, With<TileType>>,
    mut treasures_query: Query<(&mut GridPosition, &mut Transform, &Treasure), Without<TileType>>,
    selected_board: Res<SelectedBoard>,
) {
    // If the external tile had a treasure on it, move that treasure along with the external tile

    let (max_x, max_y) = get_max_coords(&selected_board);

    for tile_grid_pos in &tiles_query {
        if pos_is_external(tile_grid_pos, max_x, max_y) {
            // This is the external tile
            for (mut treasure_grid_pos, mut treasure_transform, _treasure) in &mut treasures_query {
                if pos_is_external(&treasure_grid_pos, max_x, max_y) {
                    // This is the external treasure

                    *treasure_grid_pos = *tile_grid_pos;
                    // Make it follow the external tile
                    treasure_transform.translation = Vec3::new(
                        tile_grid_pos.x_pos as f32 * TILE_SIZE.x * TILE_SCALE.x,
                        tile_grid_pos.y_pos as f32 * TILE_SIZE.y * TILE_SCALE.y,
                        2.0,
                    )
                }
            }
        }
    }
}

fn collect_treasure(
    mut commands: Commands,
    player_query: Query<(&Player, &GridPosition)>,
    treasure_query: Query<(&Treasure, Entity, &GridPosition)>,
    mut treasure_lists: ResMut<TreasureLists>,
    mut collected_lists: ResMut<CollectedLists>,
    mut game_state: ResMut<GameState>,
) {
    if !game_state.has_ended {
        // don't do any collection if game has ended
        for (player, p_grid_pos) in &player_query {
            if player.id == game_state.current_player_id {
                // Cannot collect a treasure outside of a player's turn
                for (treasure, t_entity, t_grid_pos) in &treasure_query {
                    if p_grid_pos == t_grid_pos {
                        let player_treasure_list = treasure_lists.lists.get(&player.id).unwrap();
                        if player_treasure_list
                            .iter()
                            .position(|&id| id == treasure.id)
                            == Some(player_treasure_list.len() - 1)
                        {
                            // if this is the treasure to be collected for this player,
                            // collect it and stop moving
                            commands.entity(t_entity).despawn_recursive();
                            let just_collected = treasure_lists
                                .lists
                                .get_mut(&player.id)
                                .unwrap()
                                .pop()
                                .unwrap();
                            collected_lists
                                .lists
                                .get_mut(&player.id)
                                .unwrap()
                                .push(just_collected);
                            end_turn(&mut game_state, &collected_lists);
                        }
                    }
                }
            }
        }
    }
}

fn display_current_treasure(
    mut commands: Commands,
    treasure_query: Query<(&Treasure, Entity)>,
    treasure_lists: Res<TreasureLists>,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    sprite_paths: Res<TreasureSprites>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let id = *treasure_lists
            .lists
            .get(&game_state.current_player_id)
            .unwrap()
            .last()
            .unwrap();
        spawn_treasure(
            -1,
            -2,
            2,
            &mut commands,
            &asset_server,
            &sprite_paths.list[id as usize],
            true,
        );
    } else if keys.just_released(KeyCode::Space) {
        for (treasure, entity) in &treasure_query {
            if treasure.id == -1 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn cleanup_treasures(mut commands: Commands, treasure_query: Query<Entity, With<Treasure>>) {
    for entity in &treasure_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn reset_lists(
    mut treasure_lists: ResMut<TreasureLists>,
    mut collected_lists: ResMut<CollectedLists>,
) {
    *treasure_lists = TreasureLists { ..default() };
    *collected_lists = CollectedLists { ..default() };
}
