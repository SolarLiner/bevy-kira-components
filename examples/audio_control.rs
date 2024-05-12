//! This example illustrates how to load and play an audio file.
//! For loading additional audio formats, you can enable the corresponding feature for that audio format.

use bevy::prelude::*;
use kira::effect::delay::{DelayBuilder, DelayHandle};
use kira::effect::filter::{FilterBuilder, FilterHandle};
use kira::track::TrackBuilder;

use bevy_kira_components::kira::sound::{PlaybackRate, PlaybackState};
use bevy_kira_components::kira::tween::Tween;
use bevy_kira_components::prelude::*;
use bevy_kira_components::{EffectRack, TrackBuilderWrapped};
use bevy_kira_components_derive::EffectRack;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AudioPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_speed, pause, volume))
        .run();
}

#[derive(EffectRack)]
struct MyEffectRack {
    delay: DelayBuilder,
    filter: FilterBuilder,
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioFileBundle {
            source: asset_server.load("Windless Slopes.ogg"),
            ..default()
        },
        MyEffectRack {
            delay: DelayBuilder::new(),
            filter: FilterBuilder::new().cutoff(1000.0),
        }
        .apply(TrackBuilder::new()),
    ));
}

#[derive(Component)]
struct MyMusic;

fn update_speed(
    mut music_controller: Query<&mut AudioHandle<AudioFileHandle>, With<MyMusic>>,
    time: Res<Time>,
) {
    if let Ok(mut control) = music_controller.get_single_mut() {
        let factor = ((time.elapsed_seconds() / 5.0).sin() + 1.0).max(0.1);
        control.set_playback_rate(PlaybackRate::Factor(factor as f64), Tween::default());
    }
}

fn pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut music_controller: Query<&mut AudioHandle<AudioFileHandle>, With<MyMusic>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(mut control) = music_controller.get_single_mut() {
            match control.playback_state() {
                PlaybackState::Playing => {
                    control.pause(Tween::default());
                }
                PlaybackState::Pausing | PlaybackState::Paused => {
                    control.resume(Tween::default());
                }
                PlaybackState::Stopping | PlaybackState::Stopped => {}
            }
        }
    }
}

fn volume(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut music_controller: Query<&mut AudioHandle<AudioFileHandle>, With<MyMusic>>,
    mut target_volume: Local<f64>,
) {
    if let Ok(mut control) = music_controller.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Equal) {
            *target_volume += 0.1;
            control.set_volume(*target_volume + 1.0, Tween::default());
        } else if keyboard_input.just_pressed(KeyCode::Minus) {
            *target_volume -= 0.1;
            control.set_volume(*target_volume + 1.0, Tween::default());
        }
    }
}
