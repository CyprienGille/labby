use bevy::prelude::*;

use crate::phases::GameState;

const FONT_SIZE: f32 = 35.0;

#[derive(Debug)]
enum Language {
    French,
    English,
}

#[derive(Resource, Debug)]
struct ChosenLanguage {
    lang: Language,
}

#[derive(Component, Debug)]
pub struct ControlsText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChosenLanguage {
            lang: Language::French,
        })
        .add_systems(OnEnter(GameState::Playing), spawn_controls_text)
        .add_systems(OnExit(GameState::Playing), cleanup_controls_text);
    }
}

fn spawn_controls_text(mut commands: Commands, language: Res<ChosenLanguage>) {
    let text_style = TextStyle {
        font_size: FONT_SIZE,
        color: Color::GOLD,
        ..default()
    };

    let ui_style = Style {
        position_type: PositionType::Absolute,
        left: Val::VMax(0.5),
        top: Val::VMin(1.0),
        ..default()
    };

    match language.lang {
        Language::French => {
            commands.spawn((TextBundle::from_section(
                "Fleches: Deplacer Tuile/Joueur\nR: Rotation de Tuile\nS: Simuler un deplacement de Tuile\nEntree: Pousser la Tuile\nT: Terminer son tour\nEspace: Afficher le tresor actuel\nPageUp/PageDown: Zoom",
                text_style,
            ).with_style(ui_style), ControlsText));
        }
        Language::English => {
            commands.spawn((TextBundle::from_section(
                "Arrows: Move Tile/Player\nR: Rotate Tile\nS: Simulate a push\nReturn: Push Tile\nT: End turn\nSpace: Display current treasure\nPgUp/PgDown: Zoom",
                text_style,
            ).with_style(ui_style), ControlsText));
        }
    }
}

fn cleanup_controls_text(mut commands: Commands, text_query: Query<Entity, With<ControlsText>>) {
    for entity in &text_query {
        commands.entity(entity).despawn_recursive();
    }
}
