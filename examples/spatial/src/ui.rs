use bevy::prelude::*;
use crate::{Doppler, DopplerUI};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_init).add_systems(Update, update_ui_doppler);
    }
}

fn ui_init(mut commands: Commands) {
    let style = TextStyle {
        font_size: 14.0,
        ..default()
    };
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(8.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|children| {
            let mut child = |text: &str| {
                children.spawn(TextBundle {
                    text: Text::from_section(text, style.clone()),
                    ..default()
                });
            };

            child("Click on window to lock mouse and allow movement");
            child("Use WASD or arrows to move");
            child("Use mouse to look");
            child("Press Escape to release mouse");
            children.spawn((
                DopplerUI,
                TextBundle {
                    text: Text::from_sections([
                        TextSection::new("Doppler factor ", style.clone()),
                        TextSection::new("1.00x", style.clone())
                    ]),
                    ..default()
                }
            ));
        });
}

fn update_ui_doppler(mut q_text: Query<&mut Text, With<DopplerUI>>, q_doppler: Query<&Doppler>) {
    let mut text = q_text.single_mut();
    let doppler = q_doppler.single().0;
    text.sections[1].value = format!("{doppler:1.2}x");
}