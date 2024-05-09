use std::{convert::Infallible, f32::consts::TAU};

use bevy::prelude::*;
use bevy_kira_components::sources::AudioSource;
use bevy_kira_components::{
    kira::{
        self,
        manager::error::PlaySoundError,
        sound::{Sound, SoundData},
        tween::{Parameter, Tween, Value},
    },
    prelude::*,
    sources::AudioSourcePlugin,
};
use ringbuf::{HeapConsumer, HeapProducer, HeapRb};

pub struct SineWavePlugin;

impl Plugin for SineWavePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioSourcePlugin::<SineWave>::default());
    }
}

/// Enum for commands the Handle (controlled within Bevy systems) can send to the sound (in the
/// audio thread).
///
/// This is necessary because we cannot directly control the sound from bevy (due to them being in
/// different threads). Several techniques for inter-thread communications exist, all with their
/// advantages and drawbacks. It is also necessary that those methods do not allocate or require
/// locking on the audio thread, as this would cause glitches.
///
/// In this particular case we are using message passing, sending commands from other threads to
/// this one by way of a command enum (this one), sending them in a ring-buffer so that neither the
/// audio thread nor the sending thread has to wait on each other.
enum SineWaveCommand {
    /// Set the frequency to a new value. It will use the provided `Tween` to transition from the
    /// old value to this one.
    SetFrequency(Value<f32>, Tween),
}

/// Implementation of [`Sound`] that generates a sine wave at the given frequency.
struct SineWaveSound {
    /// Output destination. This tells kira where to route the audio data that the output of our
    /// `Sound` implementation
    output: kira::OutputDestination,
    /// Commands receiver (aka. a consumer) of commands sent from other threads
    commands: HeapConsumer<SineWaveCommand>,
    /// Sine wave frequency (in Hz). Reuses `kira`'s [`Parameter`] struct to provide
    /// click-free changes and ability to provide modulations.
    frequency: Parameter<f32>,
    /// Internal phase of the sine wave. We keep track of the phase instead of the time, as this
    /// allows to modulate the frequency without glitches.
    phase: f32,
}

impl Sound for SineWaveSound {
    fn output_destination(&mut self) -> kira::OutputDestination {
        self.output
    }

    fn process(
        &mut self,
        dt: f64,
        clock_info_provider: &kira::clock::clock_info::ClockInfoProvider,
        modulator_value_provider: &kira::modulator::value_provider::ModulatorValueProvider,
    ) -> kira::Frame {
        // Receive and perform commands
        while let Some(command) = self.commands.pop() {
            match command {
                SineWaveCommand::SetFrequency(freq, tween) => self.frequency.set(freq, tween),
            }
        }

        // Compute next sample of the sine wave
        self.frequency
            .update(dt, clock_info_provider, modulator_value_provider);
        let step = self.frequency.value() * dt as f32;
        self.phase += step;
        if self.phase > 1. {
            self.phase -= 1.;
        }
        // 24 dB reduction to not blast the user's speakers (and ears)
        let sample = 0.125 * f32::sin(TAU * self.phase);

        // Return the new stereo sample
        kira::Frame {
            left: sample,
            right: sample,
        }
    }

    fn finished(&self) -> bool {
        false
    }
}

impl SineWaveSound {
    /// Create a new [`SineWaveSound`] with the provided command buffer and frequency
    fn new(
        commands: HeapConsumer<SineWaveCommand>,
        output: kira::OutputDestination,
        initial_frequency: f32,
    ) -> Self {
        Self {
            output,
            commands,
            frequency: Parameter::new(Value::Fixed(initial_frequency), initial_frequency),
            phase: 0.,
        }
    }
}

/// Handle for sine wave sounds. Allows setting the frequency.
pub struct SineWaveHandle {
    commands: HeapProducer<SineWaveCommand>,
}

impl SineWaveHandle {
    pub fn set_frequency(&mut self, frequency: impl Into<Value<f32>>, tween: Tween) {
        if self.commands.is_full() {
            error!("Cannot send command: command queue is full");
            return;
        }
        assert!(self
            .commands
            .push(SineWaveCommand::SetFrequency(frequency.into(), tween))
            .is_ok());
    }
}

/// Data and settings for the sine wave sound.
#[derive(Debug, Copy, Clone, Asset, TypePath)]
struct SineWaveData {
    /// Output destination, for consumption by the sound in `kira`
    output_destination: kira::OutputDestination,
    /// Initial frequency of the sine wave at creation
    intial_frequency: f32,
}

impl SoundData for SineWaveData {
    type Error = Infallible;

    type Handle = SineWaveHandle;

    fn into_sound(self) -> Result<(Box<dyn Sound>, Self::Handle), Self::Error> {
        let (producer, consumer) = HeapRb::new(16).split();
        let sound = Box::new(SineWaveSound::new(
            consumer,
            self.output_destination,
            self.intial_frequency,
        ));
        let handle = SineWaveHandle { commands: producer };
        Ok((sound, handle))
    }
}

#[derive(Debug, Default, Component)]
pub struct SineWaveSettings {
    pub frequency: f32,
}

/// Bevy asset for sine waves. Contains no data as the frequency is provided as a setting instead.
#[derive(Debug, Clone, Copy, Default, Asset, TypePath)]
pub struct SineWave;

impl AudioSource for SineWave {
    type Error = PlaySoundError<Infallible>;

    type Handle = SineWaveHandle;

    type Settings = SineWaveSettings;

    fn create_handle(
        &self,
        manager: &mut kira::manager::AudioManager<AudioBackend>,
        settings: &Self::Settings,
        output_destination: kira::OutputDestination,
    ) -> Result<Self::Handle, Self::Error> {
        manager.play(SineWaveData {
            intial_frequency: settings.frequency,
            output_destination,
        })
    }
}
