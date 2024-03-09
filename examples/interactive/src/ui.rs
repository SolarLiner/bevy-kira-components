use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_init);
    }
}

fn ui_init(mut commands: Commands) {
    commands
        .spawn(NodeBundle { ..default() })
        .with_children(|children| {
            children.spawn(TextBundle {
                text: Text::from_section(
                    "Hold Space to play sound",
                    TextStyle {
                        font_size: 14.0,
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                ..default()
            });
        });
}
