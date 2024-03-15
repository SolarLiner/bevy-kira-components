//! Add audio support to Bevy through the [`kira`] crate.
//!
//! This crate aims at creating a replacement for `bevy_audio` by instead integrating Kira, a crate
//! for audio playback aimed at games, and used in several other Rust projects.
//!
//! This particular crate is an experiment in making a component-based ECS API, instead of a
//! resource-based approach, currently taken by `bevy_kira_audio`.
//!
//! To get started playing sounds, insert the [`Audio`] component on an entity. The entity will
//! start playing right away unless [`Audio::start_paused`] is called.
//!
//! The [`Audio`] component requires an [`AudioFile`], an [`Asset`] which
//! supports both loading in-memory and streaming the file (only for local assets).
//!
//! Spatial audio is supported built-in with the [`SpatialEmitter`] component, which tells the plugin
//! to add the entity as an emitter, provided it also has a [`GlobalTransform`] component attached.
use bevy::prelude::*;
use bevy::transform::TransformSystem;
pub use kira;
use kira::manager::{AudioManager, AudioManagerSettings};

use crate::backend::AudioBackend;
use crate::diagnostics::KiraStatisticsDiagnosticPlugin;
use crate::sources::audio_file::AudioFilePlugin;
use crate::spatial::SpatialAudioPlugin;
use crate::tracks::AudioTracksPlugin;

mod backend;
pub mod diagnostics;
pub mod sources;
pub mod spatial;
pub mod tracks;

#[doc(hidden)]
pub mod prelude {
    pub use super::{AudioPlaybackSet, AudioPlugin, AudioSettings, AudioWorld};
    pub use crate::sources::prelude::*;
    pub use crate::spatial::prelude::*;
    pub use crate::tracks::prelude::*;
}

/// Type of settings for the audio engine. Insert it as a resource before adding the plugin to
/// change the default settings.
pub type AudioSettings = AudioManagerSettings<AudioBackend>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct AudioSourceSetup;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemSet)]
pub enum AudioPlaybackSet {
    Setup,
    Update,
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
                KiraStatisticsDiagnosticPlugin,
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
/// As a user, you'll want to use this to query information on the current audio engine settings, as
/// well as properties on audio entities.
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