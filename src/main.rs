use std::f32::consts::PI;

use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const TILE_SCALE: Vec3 = Vec3::new(0.2, 0.2, 0.0);
const TILE_SIZE: Vec3 = Vec3::new(1152.0, 1152.0, 0.0);
const NUM_TILES_X: i32 = 4;
const NUM_TILES_Y: i32 = 4;

const FIXED_POSITIONS: [(i32, i32); 2] = [(0, 0), (4, 4)];

const EXPLORER_SCALE: Vec3 = Vec3::new(50.0, 50.0, 0.0);

struct InputCooldown(Timer);

struct PushingPhase(bool);

#[derive(Component, Default)]
struct Tile {
    top: bool,
    bottom: bool,
    right: bool,
    left: bool,
    fixed: bool,
}

#[derive(Component, PartialEq)]
struct GridPosition {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Velocity;

#[derive(Component)]
struct Explorer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(InputCooldown(Timer::from_seconds(0.2, true)))
        .insert_resource(PushingPhase(true))
        .add_startup_system(setup)
        .add_system(handle_input)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Camera
    commands.spawn().insert_bundle(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(
                (NUM_TILES_X - 1) as f32 * TILE_SCALE.x * TILE_SIZE.x / 2.0,
                (NUM_TILES_Y - 1) as f32 * TILE_SCALE.y * TILE_SIZE.y / 2.0,
                10.0,
            ),
            scale: Vec3::new(2.0, 2.0, 1.0),
            ..default()
        },
        ..default()
    });

    //Tiles
    spawn_all_tiles(&mut commands, &asset_server);

    let x_pos = 0;
    let y_pos = 0;
    // Explorer 1
    commands
        .spawn()
        .insert(Explorer)
        .insert(GridPosition { x: x_pos, y: y_pos })
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.7, 0.2),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(
                    x_pos as f32 * EXPLORER_SCALE.x,
                    y_pos as f32 * EXPLORER_SCALE.y,
                    1.0,
                ),
                scale: EXPLORER_SCALE,
                ..default()
            },
            ..default()
        });
}

fn spawn_tile(
    x_pos: i32,
    y_pos: i32,
    tile_type: Option<&String>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let fixed = FIXED_POSITIONS.contains(&(x_pos, y_pos));

    let tt = match tile_type {
        Some(v) => v.to_string(),
        None => {
            // default behavior
            "T_shape".to_string()
        }
    };
    let mut bottom = false;
    let mut top = false;
    let mut right = false;
    let mut left = false;
    if tt == *"T_shape" {
        bottom = true;
        right = true;
        left = true;
    } else if tt == *"corner" {
        bottom = true;
        left = true;
    }

    let tile_entity = commands
        .spawn()
        .insert(GridPosition { x: x_pos, y: y_pos })
        .insert(Tile {
            bottom,
            top,
            right,
            left,
            fixed,
        })
        .insert_bundle(SpriteBundle {
            texture: asset_server.load(&format!("../assets/{}.png", tt)),
            transform: Transform {
                translation: Vec3::new(
                    x_pos as f32 * TILE_SIZE.x * TILE_SCALE.x,
                    y_pos as f32 * TILE_SIZE.y * TILE_SCALE.y,
                    0.0,
                ),
                scale: TILE_SCALE,
                ..default()
            },
            ..default()
        })
        .id();

    if fixed {
        println!("This tile is fixed!");
        let child = commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("../assets/fixed.png"),
                transform: Transform {
                    translation: Vec3::new(
                        x_pos as f32 * TILE_SIZE.x * TILE_SCALE.x,
                        y_pos as f32 * TILE_SIZE.y * TILE_SCALE.y,
                        0.5,
                    ),
                    scale: TILE_SCALE,
                    ..default()
                },
                ..default()
            })
            .id();
        commands.entity(tile_entity).push_children(&[child]);
    }
}

fn spawn_all_tiles(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let choices = vec!["corner".to_string(), "T_shape".to_string()];
    for x_pos in 0..NUM_TILES_X {
        for y_pos in 0..NUM_TILES_Y {
            spawn_tile(
                x_pos,
                y_pos,
                choices.choose(&mut thread_rng()),
                commands,
                asset_server,
            );
        }
    }
    spawn_tile(-1, 0, Some(&"corner".to_string()), commands, asset_server);
}

fn rotate_tile(mut tile: Mut<Tile>, mut transform: Mut<Transform>) {
    // rotates a tile 90 degrees clock-wise

    // rotate the 'open' booleans
    let temp = tile.top;
    tile.top = tile.left;
    tile.left = tile.bottom;
    tile.bottom = tile.right;
    tile.right = temp;

    //rotate the sprite
    transform.rotate_axis(Vec3::new(0.0, 0.0, 1.0), -PI / 2.0)
}

fn handle_input(
    mut pushing_phase: ResMut<PushingPhase>,
    time: Res<Time>,
    mut timer: ResMut<InputCooldown>,
    keyboard_input: Res<Input<KeyCode>>,
    mut explorer_query: Query<(&mut GridPosition, &mut Transform), With<Explorer>>,
    mut tiles_query: Query<(&mut Tile, &mut GridPosition, &mut Transform), Without<Explorer>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let (mut explorer_position, mut transform) = explorer_query
            .get_single_mut()
            .expect("Query gave more than one explorer!");
        if pushing_phase.0 {
            // tile pushing phase

            let mut final_pos_x = 0;
            let mut final_pos_y = 0;
            let mut has_reached_final_pos = false;
            for (tile, mut pos, mut transform) in &mut tiles_query {
                let horiz_ok = pos.y == -1 || pos.y == NUM_TILES_Y;
                let vertic_ok = pos.x == -1 || pos.x == NUM_TILES_X;
                if horiz_ok || vertic_ok {
                    // The movable tile
                    if keyboard_input.pressed(KeyCode::Left) && horiz_ok {
                        pos.x -= 1;
                        // move the movable tile one tile to the left
                        transform.translation.x -= TILE_SCALE.x * TILE_SIZE.x;
                    } else if keyboard_input.pressed(KeyCode::Right) && horiz_ok {
                        pos.x += 1;
                        transform.translation.x += TILE_SCALE.x * TILE_SIZE.x;
                    } else if keyboard_input.pressed(KeyCode::Up) && vertic_ok {
                        pos.y += 1;
                        transform.translation.y += TILE_SCALE.y * TILE_SIZE.y;
                    } else if keyboard_input.pressed(KeyCode::Down) && vertic_ok {
                        pos.y -= 1;
                        transform.translation.y -= TILE_SCALE.y * TILE_SIZE.y;
                    } else if keyboard_input.pressed(KeyCode::Return) {
                        final_pos_x = pos.x;
                        final_pos_y = pos.y;
                        has_reached_final_pos = true;
                    } else if keyboard_input.pressed(KeyCode::Tab) {
                        rotate_tile(tile, transform);
                    }
                }
            }
            if has_reached_final_pos {
                if final_pos_x == -1 {
                    for (tile, mut other_pos, mut other_transform) in &mut tiles_query {
                        if final_pos_y == other_pos.y && !tile.fixed {
                            // Move right all tiles in this line
                            other_transform.translation.x += TILE_SCALE.x * TILE_SIZE.x;
                            other_pos.x += 1;
                            pushing_phase.0 = false;
                        }
                    }
                } else if final_pos_x == NUM_TILES_X {
                    for (tile, mut other_pos, mut other_transform) in &mut tiles_query {
                        if final_pos_y == other_pos.y && !tile.fixed {
                            // Move left all tiles in this line
                            other_transform.translation.x -= TILE_SCALE.x * TILE_SIZE.x;
                            other_pos.x -= 1;
                            pushing_phase.0 = false;
                        }
                    }
                } else if final_pos_y == -1 {
                    for (tile, mut other_pos, mut other_transform) in &mut tiles_query {
                        if final_pos_x == other_pos.x && !tile.fixed {
                            // Move up all tiles in this column
                            other_transform.translation.y += TILE_SCALE.y * TILE_SIZE.y;
                            other_pos.y += 1;
                            pushing_phase.0 = false;
                        }
                    }
                } else if final_pos_y == NUM_TILES_Y {
                    for (tile, mut other_pos, mut other_transform) in &mut tiles_query {
                        if final_pos_x == other_pos.x && !tile.fixed {
                            // Move down all tiles in this column
                            other_transform.translation.y -= TILE_SCALE.y * TILE_SIZE.y;
                            other_pos.y -= 1;
                            pushing_phase.0 = false;
                        }
                    }
                }
            }
        } else {
            // Movement phase

            let mut destination = GridPosition { x: 0, y: 0 };
            let mut from_left = false;
            let mut from_right = false;
            let mut from_top = false;
            let mut from_bottom = false;
            if keyboard_input.pressed(KeyCode::Left) {
                println!("Pressed Left!");
                destination = GridPosition {
                    x: explorer_position.x - 1,
                    y: explorer_position.y,
                };
                from_right = true;
            } else if keyboard_input.pressed(KeyCode::Right) {
                println!("Pressed Right!");
                destination = GridPosition {
                    x: explorer_position.x + 1,
                    y: explorer_position.y,
                };
                from_left = true;
            } else if keyboard_input.pressed(KeyCode::Up) {
                println!("Pressed Up!");
                destination = GridPosition {
                    x: explorer_position.x,
                    y: explorer_position.y + 1,
                };
                from_bottom = true;
            } else if keyboard_input.pressed(KeyCode::Down) {
                println!("Pressed Down!");
                destination = GridPosition {
                    x: explorer_position.x,
                    y: explorer_position.y - 1,
                };
                from_top = true;
            } else if keyboard_input.pressed(KeyCode::Return) {
                pushing_phase.0 = true;
            }

            let mut dest_top = false;
            let mut dest_bottom = false;
            let mut dest_right = false;
            let mut dest_left = false;
            let mut current_top = false;
            let mut current_bottom = false;
            let mut current_right = false;
            let mut current_left = false;
            for (tile, pos, _) in &tiles_query {
                if pos == &destination {
                    dest_top = tile.top;
                    dest_bottom = tile.bottom;
                    dest_right = tile.right;
                    dest_left = tile.left;
                } else if pos.x == explorer_position.x && pos.y == explorer_position.y {
                    current_top = tile.top;
                    current_bottom = tile.bottom;
                    current_right = tile.right;
                    current_left = tile.left;
                }
            }
            if from_top && dest_top && current_bottom {
                println!("Entered Tile from top!");
                transform.translation.y -= TILE_SIZE.y * TILE_SCALE.y;
                explorer_position.y -= 1;
            } else if from_bottom && dest_bottom && current_top {
                println!("Entered Tile from bottom!");
                transform.translation.y += TILE_SIZE.y * TILE_SCALE.y;
                explorer_position.y += 1;
            } else if from_right && dest_right && current_left {
                println!("Entered Tile from right!");
                transform.translation.x -= TILE_SIZE.x * TILE_SCALE.x;
                explorer_position.x -= 1;
            } else if from_left && dest_left && current_right {
                println!("Entered Tile from left!");
                transform.translation.x += TILE_SIZE.x * TILE_SCALE.x;
                explorer_position.x += 1;
            } else {
                println!("No input/No valid input");
            }
        }
    }
}
