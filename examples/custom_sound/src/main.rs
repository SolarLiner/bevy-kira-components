use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_components::sources::AudioBundle;
use bevy_kira_components::{kira::tween::Tween, prelude::*};
use diagnostics_ui::DiagnosticsUiPlugin;
use sine_wave::{SineWave, SineWaveHandle, SineWavePlugin, SineWaveSettings};

mod sine_wave;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AudioPlugin,
            SineWavePlugin,
            DiagnosticsUiPlugin,
        ))
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, add_sounds)
        .add_systems(PostUpdate, control_sounds)
        .run();
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct MySine;

fn add_sounds(mut commands: Commands, mut sine_waves: ResMut<Assets<SineWave>>) {
    info!("Spawning entity with sine wave bundle");
    commands.spawn((
        MySine,
        AudioBundle {
            source: sine_waves.add(SineWave),
            settings: SineWaveSettings { frequency: 440.0 },
            ..default()
        },
    ));
}

fn control_sounds(
    mut q: Query<
        &mut AudioHandle<SineWaveHandle>,
        (With<MySine>, Added<AudioHandle<SineWaveHandle>>),
    >,
) {
    let Ok(mut handle) = q.get_single_mut() else {
        return;
    };
    info!("Sending command to sine wave sound");
    handle.set_frequency(
        1000.0,
        Tween {
            duration: Duration::from_secs(1),
            ..default()
        },
    );
}
