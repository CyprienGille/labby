use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const TILE_SCALE: Vec3 = Vec3::new(0.2, 0.2, 0.0);
const TILE_SIZE: Vec3 = Vec3::new(1152.0, 1152.0, 0.0);

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(InputCooldown(Timer::from_seconds(1.0, true)))
        .add_startup_system(setup)
        .add_system(handle_input)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Camera
    commands.spawn_bundle(Camera2dBundle::default());

    //Tile
    for x_pos in 0..2 {
        for y_pos in 0..2 {
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

        for (tile, pos) in &tiles_query {
            if pos == &destination {
                if from_top && tile.top {
                    println!("Entered Tile from top!");
                    transform.translation.y -= TILE_SIZE.y * TILE_SCALE.y;
                } else if from_bottom && tile.bottom {
                    println!("Entered Tile from bottom!");
                    transform.translation.y += TILE_SIZE.y * TILE_SCALE.y;
                } else if from_right && tile.right {
                    println!("Entered Tile from right!");
                    transform.translation.x -= TILE_SIZE.x * TILE_SCALE.x;
                } else if from_left && tile.left {
                    println!("Entered Tile from left!");
                    transform.translation.x += TILE_SIZE.x * TILE_SCALE.x;
                } else {
                    println!("No input/No valid input");
                }
            }
        }
    }
}
