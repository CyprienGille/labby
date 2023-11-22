use bevy::prelude::*;

#[derive(Debug, Default, Component, Clone, Copy)]
pub enum CanMove {
    #[default]
    Yes,
    No,
}
