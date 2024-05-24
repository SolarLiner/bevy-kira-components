//! # Custom audio sources
//!
//! ## A solemn forewarning
//!
//! Hark, ye who tread the path of the auditory arts, heed this scroll's solemn words, for they
//! unveil the treacherous pitfalls lurking within the realm of audio programming. As you embark on
//! your quest to weave procedural sound effects into the tapestry of existence, beware the siren
//! song of complexity that ensnares the unwary programmer. Beneath the guise of enchanting harmonies
//! lies the labyrinthine maze of real-time programming, where even the most seasoned minstrels may
//! lose themselves amidst the tangled webs of treacherous ways.
//!
//! Venture forth with vigilance, for in the realm of audio programming lies the challenge of
//! minimizing allocations. Beware the temptation to squander precious resources in the pursuit of
//! convenience, for each unnecessary allocation is a shackle binding thy creations to the
//! dreaded behemoth of thread scheduling. Strive instead to compose thy code with the elegance
//! of a virtuoso, orchestrating symphonies that sing with the clarity of unburdened thread
//! execution.
//!
//! As you seek to craft your incantations that transcend the limitations of earthly constraints,
//! embrace the art of wait-free and lock-free programming. Let not thy magic be marred by the
//! discord of contention and delay, but instead, harmonize thy algorithms with the rhythm of
//! seamless execution. With patience and precision, mayhaps ye shall unlock the secrets of audio
//! programming's true potential, weaving melodies that echo through the ages.
//!
//! Finally, let not thy pride blind thee to the wisdom of collaboration, for in unity lies strength.
//! Though the solitary path may seem enticing, remember that no bard is an island unto themselves.
//! Seek counsel from fellow travelers versed in the arcane arts of memory management and concurrency
//! control, and together, mayhaps ye shall compose masterpieces that resound with the echoes of
//! eternity.
//!
//! Behold, for a tome of further wisdom awaits your perusal [here]. Inscribe these words upon your
//! heart, and let them kindle the fires of resilience within. Tread this path at your own rhythm,
//! for it is not the speed of the journey, but the steadfastness of purpose that shall empower thee.
//!
//! [here]: http://www.rossbencina.com/code/real-time-audio-programming-101-time-waits-for-nothing
//!
//! # Implementing a custom sound
//!
//! ## Audio generation
//!
//! The most basic custom audio source implementation is to simply implement [`Sound`], which
//! is where the actual audio generation will happen. Every time this function gets called, a new
//! pair of stereo audio samples (called a `Frame` in Kira) is generated, which corresponds to
//! the position of the speaker cone at that instant. Much like physics, for example, sound is
//! computed at regular intervals, though much faster than physics, since it usually runs at
//! around 44-48 kHz. Each sample, then, only represent a few microseconds of time.
//!
//! ## Control
//!
//! To be useful in the vast majority of cases, you'll want your sound generation to be
//! controllable from the outside world (i.e. the ECS). This is where things get tricky, because a
//! lot of solutions that exist for inter-thread communications (i.e. channels, mutexes, etc.) are
//! invalid within the realm of real-time programming. Instead, the most common solution is to
//! send events or commands through a fixed-size ring buffer.
//!
//! This is the solution used within Kira, and here below. the various setter methods that you
//! will implement will not directly change the value in the [`Sound`] implementation, but
//! instead create a command indicating the change, to be sent to the audio thread through the
//! ring-buffer.
//!
//! Then, on the audio side, within the [`Sound`] implementation, the ring-buffer is consumed for
//! any sent commands to be applied. You can easily send raw values, as well as Kira modulators
//! and tweens, and Kira provides a [`Parameter`] struct which seamlessly integrates with those,
//! in such a way that you only have to consume the events and pass the values to the parameter.
//!
//! To clearly mark the separation of concerns, as well as the ownership of each half by
//! different threads, the audio processing-side type (here [`SineWaveSound`]) and the ECS-side
//! type (here [`SineWaveHandle`]) are separate, and only require access to their half of the
//! ring-buffer (which, here, is seen as a single-producer, single-consumer queue). The handle
//! pushes commands (here of type [`SineWaveCommand`]) into the ring-buffer, and the sound pulls
//! from it and applies the commands.
//!
//! Note that there is nothing to implement for the handle, as it is entirely dependent on the
//! applicable parameters and
//!
//! Most of the code of the handle is to provide a nice user-facing API, which hides the
//! ring-buffer as an implementation detail.
//!
//! ## Data
//!
//! In order to be constructed, you'll most probably require your user to provide initial
//! value, data (such as audio data from samples), as well as settings related to routing the audio
//! to a track or to a spatial scene. This "settings type" should implement [`SoundData`], whose
//! only job is to construct the [`Sound`] and its associated handle, so that Kira knows how to
//! work with your custom sound implementation. This type is not going to be used by end-users,
//! it's only the middle step in bridging the ECS and the audio engine.
//!
//! ## Settings
//!
//! TODO: Writing this triggered some existential crisis over the API 😵‍💫
//!
//! ## Usage in the ECS
//!
//! TODO: Writing this triggered some existential crisis over the API 😵‍💫

use std::convert::Infallible;
use std::f32::consts::TAU;

use bevy::prelude::*;
use kira::manager::error::PlaySoundError;
use kira::sound::{Sound, SoundData};
use kira::tween::{Parameter, Tween, Value};
use ringbuf::{HeapConsumer, HeapProducer, HeapRb};

use bevy_kira_components::prelude::*;
// TODO: ambiguity is with bevy which still exposes `struct AudioSource` even when the `bevy_audio`
// feature is off. We'll have to break the monolithic `bevy` import anyway, so this will be solved
// then.
use bevy_kira_components::prelude::AudioSource;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AudioPlugin,
            // The audio source plugin is generic over audio sources; use it to register systems
            // that will manage your custom audio source for you.
            AudioSourcePlugin::<SineWave>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut assets: ResMut<Assets<SineWave>>) {
    use bevy_kira_components::sources::AudioBundle;
    let handle = assets.add(SineWave);

    // The `AudioBundle` is also generic over the sound source. You're probably already using it
    // through the `AudioFileBundle` alias for audio files.
    commands.spawn(AudioBundle {
        source: handle,
        settings: SineWaveSettings { frequency: 440.0 },
        ..default()
    });
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
    /// old value to t:his one.
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
        // 24 dB = 8x reduction to not blast the user's speakers (and ears)
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
            error!("maximum number of in-flight commands reached, cannot add any more");
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
