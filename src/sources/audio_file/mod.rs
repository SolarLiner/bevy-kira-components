//! Audio source implementation for audio files.
//!
//! This implementation wraps both `StaticSound` and `StreamingSound`, to allow for better defaults around streaming
//! like having a size threshold, beyond which the file is kept on disk instead of loaded into memory in its entirety.
//! (note that this is not yet implemented).
//!
//! Specifying if the asset is streamed or not is part of the loader settings, which can be changed in `.meta` files,
//! or specified with [`AssetServer::load_with_settings`].

use bevy::prelude::*;
use kira::manager::error::PlaySoundError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{AudioBundle, AudioHandle, AudioSource, AudioSourcePlugin};

pub mod loader;
pub mod source;

#[doc(hidden)]
#[allow(missing_docs)]
pub mod prelude {
    pub use super::loader::*;
    pub use super::{
        AudioFile, AudioFileBundle, AudioFileError, AudioFileHandle, AudioFileSettings,
    };
    pub use super::source::*;
}

/// Specialization of [`AudioBundle`] for the [`AudioFile`] asset.
pub type AudioFileBundle = AudioBundle<AudioFile>;

/// Implementation of an audio source using the Static and Streaming file data from [`kira`].
pub struct AudioFilePlugin;

impl Plugin for AudioFilePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<AudioFileLoader>()
            .add_plugins(AudioSourcePlugin::<AudioFile>::default())
            .add_systems(PostUpdate, audio_finished.in_set(AudioPlaybackSet::Cleanup));
    }
}

fn audio_finished(
    mut commands: Commands,
    q_sources: Query<(Entity, &AudioHandle<AudioFileHandle>)>,
) {
    for (entity, AudioHandle(handle)) in &q_sources {
        if matches!(handle.playback_state(), PlaybackState::Stopped) {
            commands
                .entity(entity)
                .remove::<AudioHandle<AudioFileHandle>>();
        }
    }
}

/// Enumeration of possible errors when loading an audio file.
#[derive(Debug, Error)]
pub enum AudioFileError {
    /// Error comes from trying to load the file for streaming
    #[error(transparent)]
    FromFileError(#[from] FromFileError),
}

/// Settings available to the user when instantiating an audio file.
#[derive(Debug, Component, Deserialize, Serialize)]
pub struct AudioFileSettings {
    /// By default, sounds will start playing right away when inserted. Setting this to `true`
    /// prevents that.
    pub start_paused: bool,
    /// Volume at which the audio will play at.
    pub volume: f64,
    /// Panning (in 0..=1) for the sound, where 0 is hard left, and 1 is hard right.
    pub panning: f64,
    /// Optionally loop a region of the sound (given in seconds)
    pub loop_region: Option<Region>,
    /// Only play a specific region of the file
    pub play_region: Region,
    /// Play the file in reverse (not available for streaming sound files)
    pub reverse: bool,
    // pub start_time: StartTime, // TODO: Implement with serializable types
}

impl Default for AudioFileSettings {
    fn default() -> Self {
        Self {
            start_paused: false,
            volume: 1.0,
            panning: 0.5,
            loop_region: None,
            play_region: Region::from(..),
            reverse: false,
        }
    }
}

fn play_sound_error_transmute<Out>(err: PlaySoundError<()>) -> PlaySoundError<Out> {
    match err {
        PlaySoundError::CommandError(cmd) => PlaySoundError::CommandError(cmd),
        PlaySoundError::SoundLimitReached => PlaySoundError::SoundLimitReached,
        _ => unreachable!(),
    }
}

fn play_sound_error_cast<In, Out: From<In>>(err: PlaySoundError<In>) -> PlaySoundError<Out> {
    match err {
        PlaySoundError::CommandError(cmd) => PlaySoundError::CommandError(cmd),
        PlaySoundError::SoundLimitReached => PlaySoundError::SoundLimitReached,
        PlaySoundError::IntoSoundError(input) => PlaySoundError::IntoSoundError(input.into()),
        _ => unreachable!(),
    }
}
