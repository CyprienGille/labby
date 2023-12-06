use bevy::prelude::*;

use crate::{
    actors::{get_random_pos_on_axis, GridAxis, SpawnPosition},
    board_selector::SelectedBoard,
    movement::CanMove,
    phases::GamePhase,
    tile::{TILE_SCALE, TILE_SIZE},
    GameSettings, GameState, GridPosition,
};

const TOKEN_SCALE: Vec3 = Vec3::new(0.4, 0.4, 0.0);
// const TOKEN_SIZE: Vec3 = Vec3::new(280.0, 280.0, 0.0);
const WIGGLE_VALUE: f32 = 20.0;

const SPRITES: [&str; 4] = [
    "players/Commoner.png",
    "players/Genie.png",
    "players/Harengon.png",
    "players/Speaker.png",
];

#[derive(Resource, Debug, Default)]
struct WiggledPlayers {
    pairs: Vec<PlayerPair>,
}

#[derive(Debug, Default, PartialEq)]
struct PlayerPair {
    id_1: i32,
    id_2: i32,
}

#[derive(Component, Debug)]
pub struct Player {
    pub id: i32,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    pos: GridPosition,
    sprite: SpriteBundle,
    can_move: CanMove,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WiggledPlayers {
            ..Default::default()
        })
        .add_systems(OnEnter(GamePhase::Playing), spawn_all_players)
        .add_systems(
            Update,
            (unstack_players, display_current_player).run_if(in_state(GamePhase::Playing)),
        )
        .add_systems(OnExit(GamePhase::Playing), cleanup_players);
    }
}

fn spawn_all_players(
    mut commands: Commands,
    game_settings: Res<GameSettings>,
    selected_board: Res<SelectedBoard>,
    asset_server: Res<AssetServer>,
) {
    // The number of specified spawn positions in the selected board's data
    let num_spawn_pos: i32 = selected_board
        .board
        .spawn_positions
        .len()
        .try_into()
        .unwrap();
    // The used spawn positions (to avoid superposition)
    let mut used_pos = vec![];

    for id in 0..game_settings.num_players {
        let mut current_spawn_pos = SpawnPosition::Any;
        if id < num_spawn_pos {
            // Get the next specified spawn position
            current_spawn_pos = selected_board.board.spawn_positions[id as usize];
        }
        match current_spawn_pos {
            // If the specified position is set, spawn player there and add it to the used pos list
            SpawnPosition::Position(GridPosition { x_pos, y_pos }) => {
                spawn_player(
                    id,
                    x_pos,
                    y_pos,
                    &mut commands,
                    &asset_server,
                    SPRITES[(id % 4) as usize],
                );
                used_pos.push(GridPosition { x_pos, y_pos });
            }
            // If the specified position is not set, pick a random position
            // without superposition if possible
            SpawnPosition::Any => {
                let mut x_pos = get_random_pos_on_axis(GridAxis::X, &selected_board);
                let mut y_pos = get_random_pos_on_axis(GridAxis::Y, &selected_board);
                // while not new position AND there are positions without players
                while used_pos.contains(&GridPosition { x_pos, y_pos })
                    && used_pos.len() < (game_settings.num_players - 1).try_into().unwrap()
                {
                    x_pos = get_random_pos_on_axis(GridAxis::X, &selected_board);
                    y_pos = get_random_pos_on_axis(GridAxis::Y, &selected_board);
                }
                spawn_player(
                    id,
                    x_pos,
                    y_pos,
                    &mut commands,
                    &asset_server,
                    SPRITES[(id % 4) as usize],
                );
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
    sprite_path: &str,
) {
    commands.spawn(PlayerBundle {
        player: Player { id },
        pos: GridPosition { x_pos, y_pos },
        sprite: SpriteBundle {
            texture: asset_server.load(sprite_path.to_owned()),
            transform: Transform {
                translation: Vec3::new(
                    x_pos as f32 * TILE_SIZE.x * TILE_SCALE.x,
                    y_pos as f32 * TILE_SIZE.y * TILE_SCALE.y,
                    1.0,
                ),
                scale: TOKEN_SCALE,
                ..default()
            },
            ..default()
        },
        can_move: CanMove::No,
    });
}

fn unstack_players(
    mut player_query: Query<(&GridPosition, &mut Transform, &Player)>,
    mut wiggled_players: ResMut<WiggledPlayers>,
) {
    let mut combinations = player_query.iter_combinations_mut();
    while let Some(
        [(grid_pos_1, mut transform_1, player_1), (grid_pos_2, mut transform_2, player_2)],
    ) = combinations.fetch_next()
    {
        let pair = PlayerPair {
            id_1: player_1.id,
            id_2: player_2.id,
        };
        let same_tile = grid_pos_1 == grid_pos_2;

        if wiggled_players.pairs.contains(&pair) {
            // if the players have been wiggled
            if !same_tile {
                // If the two players are no longer on the same tile
                transform_1.translation.x -= WIGGLE_VALUE;
                transform_2.translation.y -= WIGGLE_VALUE;
                wiggled_players.pairs.retain(|x| *x != pair);
            }
        } else if same_tile {
            // If the two players are on the same tile and have not been wiggled
            transform_1.translation.x += WIGGLE_VALUE;
            transform_2.translation.y += WIGGLE_VALUE;
            wiggled_players.pairs.push(pair);
        }
    }
}

fn display_current_player(
    mut commands: Commands,
    player_query: Query<(&Player, Entity)>,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        spawn_player(
            -1,
            -2,
            3,
            &mut commands,
            &asset_server,
            SPRITES[(game_state.current_player_id % 4) as usize],
        );
    } else if keys.just_released(KeyCode::Space) {
        for (player, entity) in &player_query {
            if player.id == -1 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn cleanup_players(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in &player_query {
        commands.entity(entity).despawn_recursive();
    }
}
