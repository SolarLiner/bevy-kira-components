use bevy::prelude::*;
use bevy::utils::error;

use bevy_kira_components::kira::sound::Region;
use bevy_kira_components::kira::tween::Tween;
use bevy_kira_components::prelude::AudioBundle;
use bevy_kira_components::prelude::*;
use diagnostics_ui::DiagnosticsUiPlugin;

use crate::ui::UiPlugin;

mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin, DiagnosticsUiPlugin, UiPlugin))
        .add_systems(Startup, init)
        .add_systems(Update, handle_interactive_sound)
        .run();
}

#[derive(Component)]
struct InteractiveSound;

#[derive(Component)]
struct PanTrack;

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle { ..default() });
    let audio_file = asset_server.load::<AudioFile>("drums.ogg");
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(25.0)),
            sprite: Sprite {
                color: Color::GRAY,
                ..default()
            },
            ..default()
        },
        AudioBundle {
            source: audio_file,
            settings: AudioFileSettings {
                loop_region: Some(Region::from(3.6..6.0)),
                start_paused: true,
                ..default()
            },
            ..default()
        },
        InteractiveSound,
    ));
}

fn handle_interactive_sound(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut AudioHandle<AudioFileHandle>, &mut Sprite), With<InteractiveSound>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for (mut handle, mut sprite) in &mut q {
            error(handle.resume(Tween::default()));
            sprite.color = Color::GREEN;
        }
    } else if keyboard.just_released(KeyCode::Space) {
        for (mut handle, mut sprite) in &mut q {
            error(handle.pause(Tween::default()));
            sprite.color = Color::GRAY;
        }
    }
}
