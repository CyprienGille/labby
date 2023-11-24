use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct FpsText;

const TEXT_SIZE: f32 = 30.0;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, spawn_fps_text)
            .add_systems(Update, show_running_info);
    }
}

fn spawn_fps_text(mut commands: Commands) {
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: TEXT_SIZE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: TEXT_SIZE,
                color: Color::GOLD,
                ..default()
            }),
        ]),
        FpsText,
    ));
}

fn show_running_info(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    // Update second text section with fps value
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
