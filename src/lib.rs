//! Add audio support to Bevy through the [`kira`] crate.
//!
//! This crate aims at creating a replacement for `bevy_audio` by instead integrating Kira, a crate
//! for audio playback aimed at games, and used in several other Rust projects.
//!
//! This particular crate is an experiment in making a component-based ECS API, instead of a
//! resource-based approach, currently taken by `bevy_kira_audio`.
//!
//! To get started playing sounds, insert an [`AudioBundle`](prelude::AudioBundle) on an entity.
//! This is a generic bundle which supports any compatible sound source. An implementation over
//! audio files is provided, with streaming support, using the
//! [`AudioFileBundle`](prelude::AudioFileBundle) type alias.
//!
//! When an [`AudioFile`](prelude::AudioFile) is inserted, its playback will begin right away. To
//! prevent that, use the [`start_paused`](prelude::AudioFileSettings) field from
//! [`AudioFileBundle::settings`](prelude::AudioFileBundle::settings) and set it to false.
//!
//! The audio system creates an [`AudioHandle`](prelude::AudioHandle) component when registering an
//! added sound for
//! playback. This handle allows you to control the sound. For [`AudioFile`](prelude::AudioFile)s,
//! this means pausing/resuming, setting the volume, panning, and playback rate of the sound.
//!
//! Spatial audio is supported built-in with the [`SpatialEmitter`](prelude::SpatialEmitter)
//! component, which tells the plugin to add the entity as an emitter, provided it also has a
//! [`GlobalTransform`] component attached. Its settings control the behavior of the spatial effect.
//!
//! ## Example
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_kira_components::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .insert_non_send_resource(AudioSettings {
//!             // Only needed for tests
//!             backend_settings: AudioBackendSelector::Mock { sample_rate: 48000, },
//!             ..default()
//!         })
//!         .add_plugins((DefaultPlugins, AudioPlugin))
//!         .add_systems(Startup, add_sound)
//!         .run();
//! }
//!
//! fn add_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
//!     commands.spawn(AudioFileBundle {
//!         source: asset_server.load("my_sound.ogg"),
//!         ..default()
//!     });
//! }
//! ```
#![warn(missing_docs)]

use bevy::prelude::*;
use bevy::transform::TransformSystem;
pub use kira;
use kira::manager::{AudioManager, AudioManagerSettings};

use crate::backend::AudioBackend;
use crate::sources::audio_file::AudioFilePlugin;
use crate::spatial::SpatialAudioPlugin;
use crate::tracks::AudioTracksPlugin;

mod backend;
pub mod diagnostics;
pub mod sources;
pub mod spatial;
pub mod tracks;

/// Prelude.
///
/// Use as `use bevy_kira_components::prelude::*;` in your own games.
pub mod prelude {
    pub use super::{AudioPlaybackSet, AudioPlugin, AudioSettings, AudioWorld};
    pub use crate::backend::*;
    pub use crate::sources::prelude::*;
    pub use crate::spatial::prelude::*;
    pub use crate::tracks::prelude::*;
}

/// Type of settings for the audio engine. Insert it as a resource before adding the plugin to
/// change the default settings.
pub type AudioSettings = AudioManagerSettings<AudioBackend>;

/// System set used in grouping systems that setup audio sources. Used in the
/// [`AudioSourcePlugin`](prelude::AudioSourcePlugin)'s systems. Useful to place systems right
/// after to be able to react to added audio assets.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct AudioSourceSetup;

/// General audio system set, used by the systems in this plugin.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemSet)]
pub enum AudioPlaybackSet {
    /// Systems related to setting up audio sources, registering them with the audio engine, and
    /// bookkeeping handles.
    Setup,
    /// Systems related to keeping the audio engine in sync with Bevy's world. Mainly used by the
    /// spatial systems to copy position and rotation information from listeners and emitters.
    Update,
    /// Systems that clean up resources. Handles that have finished playing or are no longer
    /// useful, are removed automatically here.
    Cleanup,
}

/// Adds audio to Bevy games via the [`kira`] crate.
#[derive(Debug, Default)]
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioWorld>()
            .add_plugins((
                AudioTracksPlugin,
                #[cfg(feature = "diagnostics")]
                diagnostics::KiraStatisticsDiagnosticPlugin,
                SpatialAudioPlugin,
                AudioFilePlugin,
            ))
            .configure_sets(PreUpdate, AudioPlaybackSet::Setup)
            .configure_sets(
                PostUpdate,
                AudioPlaybackSet::Update.after(TransformSystem::TransformPropagate),
            );
    }
}

/// Main resource holding all the bookkeeping necessary to link the ECS data to the audio engine.
#[derive(Resource)]
pub struct AudioWorld {
    pub(crate) audio_manager: AudioManager<AudioBackend>,
}

impl FromWorld for AudioWorld {
    fn from_world(world: &mut World) -> Self {
        let audio_manager_settings = world
            .remove_non_send_resource::<AudioSettings>()
            .unwrap_or_default();
        let audio_manager =
            AudioManager::new(audio_manager_settings).expect("Cannot create audio backend");
        Self { audio_manager }
    }
}

#[derive(Component)]
#[doc(hidden)]
/// Internal marker for entities with audio components. Needed to be able to query in a
/// non-generic way for having added audio support through the [`AudioBundle`] struct.
pub struct InternalAudioMarker;
