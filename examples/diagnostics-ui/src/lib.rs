use bevy::diagnostic::{DiagnosticPath, DiagnosticsStore};
use bevy::prelude::*;

pub struct DiagnosticsUiPlugin;

impl Plugin for DiagnosticsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, init_ui)
            .add_systems(Update, update_ui);
    }
}

#[derive(Component)]
struct DiagnosticSource(DiagnosticPath);

fn init_ui(mut commands: Commands, diagnostics: Res<DiagnosticsStore>) {
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK.with_a(0.4)),
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(1.0),
                top: Val::Percent(1.0),
                bottom: Val::Auto,
                left: Val::Auto,
                padding: UiRect::all(Val::Px(4.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|children| {
            for diag in diagnostics.iter() {
                let style = TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                };
                children.spawn((
                    DiagnosticSource(diag.path().clone()),
                    TextBundle {
                        text: Text::from_sections([
                            TextSection {
                                value: diag.path().to_string(),
                                style: style.clone(),
                            },
                            TextSection {
                                value: " N/A".into(),
                                style: style.clone(),
                            },
                            TextSection {
                                value: format!(" {}", diag.suffix),
                                style,
                            },
                        ]),
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            align_content: AlignContent::End,
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
        });
}

fn update_ui(diagnostics: Res<DiagnosticsStore>, mut q: Query<(&DiagnosticSource, &mut Text)>) {
    for (diag, mut text) in &mut q {
        if let Some(value) = diagnostics
            .get(&diag.0)
            .and_then(|d| d.is_enabled.then(|| d.value()).flatten())
        {
            text.sections[1].value = format!(" {value}");
        } else {
            text.sections[1].value = " (deactivated)".to_string();
        }
    }
}
