use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const TILE_SCALE: Vec3 = Vec3::new(0.2, 0.2, 0.0);
const TILE_SIZE: Vec3 = Vec3::new(1152.0, 1152.0, 0.0);
const NUM_TILES_X: i32 = 4;
const NUM_TILES_Y: i32 = 4;

const EXPLORER_SCALE: Vec3 = Vec3::new(50.0, 50.0, 0.0);

struct InputCooldown(Timer);

#[derive(Component, Default)]
struct Tile {
    top: bool,
    bottom: bool,
    right: bool,
    left: bool,
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

#[derive(Component)]
struct TopDownCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(InputCooldown(Timer::from_seconds(0.2, true)))
        .add_startup_system(setup)
        .add_system(handle_input)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Camera
    commands
        .spawn()
        .insert_bundle(Camera2dBundle {
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
        })
        .insert(TopDownCamera);

    //Tiles
    for x_pos in 0..NUM_TILES_X {
        for y_pos in 0..NUM_TILES_Y {
            commands
                .spawn()
                .insert(Tile {
                    left: true,
                    bottom: true,
                    right: true,
                    ..default()
                })
                .insert(GridPosition { x: x_pos, y: y_pos })
                .insert_bundle(SpriteBundle {
                    texture: asset_server.load("../assets/T_shape.png"),
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
                });
        }
    }

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

fn handle_input(
    time: Res<Time>,
    mut timer: ResMut<InputCooldown>,
    keyboard_input: Res<Input<KeyCode>>,
    mut explorer_query: Query<(&mut GridPosition, &mut Transform), With<Explorer>>,
    tiles_query: Query<(&Tile, &GridPosition), Without<Explorer>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let (mut explorer_position, mut transform) = explorer_query
            .get_single_mut()
            .expect("Query gave more than one explorer!");

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
        }
        if keyboard_input.pressed(KeyCode::Right) {
            println!("Pressed Right!");
            destination = GridPosition {
                x: explorer_position.x + 1,
                y: explorer_position.y,
            };
            from_left = true;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            println!("Pressed Up!");
            destination = GridPosition {
                x: explorer_position.x,
                y: explorer_position.y + 1,
            };
            from_bottom = true;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            println!("Pressed Down!");
            destination = GridPosition {
                x: explorer_position.x,
                y: explorer_position.y - 1,
            };
            from_top = true;
        }

        let mut dest_top = false;
        let mut dest_bottom = false;
        let mut dest_right = false;
        let mut dest_left = false;
        let mut current_top = false;
        let mut current_bottom = false;
        let mut current_right = false;
        let mut current_left = false;
        for (tile, pos) in &tiles_query {
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
