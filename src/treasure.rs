use bevy::prelude::*;

use crate::GridPosition;

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

fn spawn_all_treasures() {}
