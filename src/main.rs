use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Component, Default)]
struct Tile {
    top: bool,
    bottom: bool,
    right: bool,
    left: bool,
}

#[derive(Component)]
struct Velocity;

#[derive(Component)]
struct Explorer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Camera
    commands.spawn_bundle(Camera2dBundle::default());

    //Tile
    commands
        .spawn()
        .insert(Tile {
            left: true,
            bottom: true,
            ..default()
        })
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("../assets/corner.png"),
            transform: Transform {
                scale: Vec3::splat(0.5),
                ..default()
            },
            ..default()
        });
}
