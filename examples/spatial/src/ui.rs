use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_init);
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
        });
}
