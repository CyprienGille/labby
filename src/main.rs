mod camera;

use bevy::prelude::*;
use camera::Camera2dPlugin;

fn main() {
    App::new()
        //bevy Built-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.4, 0.1)))
        .add_plugins(DefaultPlugins)
        // User plugins
        .add_plugins(Camera2dPlugin)
        .run();
}
