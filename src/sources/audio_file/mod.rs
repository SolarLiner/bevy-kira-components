use bevy::prelude::*;
use bevy::utils::error;
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings};
use std::io::Cursor;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use crate::backend::AudioBackend;
use kira::manager::error::PlaySoundError;
use kira::manager::AudioManager;
use kira::sound::streaming::{StreamingSoundData, StreamingSoundHandle, StreamingSoundSettings};
use kira::sound::{FromFileError, PlaybackRate, PlaybackState, Region, Sound};

use crate::sources::audio_file::loader::AudioFileLoader;
use crate::AudioPlaybackSet;
use kira::tween::{Tween, Value};
use kira::{CommandError, OutputDestination, StartTime};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{AudioBundle, AudioHandle, AudioSource, AudioSourcePlugin};

pub mod loader;

#[doc(hidden)]
pub mod prelude {
    pub use super::loader::*;
    pub use super::{
        AudioFile, AudioFileBundle, AudioFileError, AudioFileHandle, AudioFileSettings,
    };
}

pub type AudioFileBundle = AudioBundle<AudioFile>;

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

/// Bevy [`Asset`] implementation that wraps audio data for [`kira`].
///
/// Streaming audio data is currently not possible over the internet, so when targeting the web,
/// all audio sources need to be [`Static`](Self::Static).
#[derive(Asset, Clone, TypePath)]
pub enum AudioFile {
    Static(Arc<[u8]>, StaticSoundSettings),
    Streaming {
        path: PathBuf,
        settings: StreamingSoundSettings,
    },
}

#[derive(Debug, Error)]
pub enum AudioFileError {
    #[error(transparent)]
    FromFileError(#[from] FromFileError),
}

#[derive(Debug, Component, Deserialize, Serialize)]
pub struct AudioFileSettings {
    pub start_paused: bool,
    pub volume: f64,
    pub panning: f64,
    pub loop_region: Option<Region>,
    pub play_region: Region,
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

impl AudioSource for AudioFile {
    type Error = PlaySoundError<AudioFileError>;
    type Handle = AudioFileHandle;
    type Settings = AudioFileSettings;

    fn create_handle(
        &self,
        manager: &mut AudioManager<AudioBackend>,
        settings: &Self::Settings,
        output_destination: OutputDestination,
    ) -> Result<Self::Handle, Self::Error> {
        let start_paused = settings.start_paused;
        match self {
            Self::Static(data, kira_settings) => {
                let settings = kira_settings
                    .clone()
                    .output_destination(output_destination)
                    .volume(settings.volume)
                    .panning(settings.panning)
                    .loop_region(settings.loop_region)
                    .reverse(settings.reverse)
                    .playback_region(settings.play_region);
                let static_data = StaticSoundData::from_cursor(Cursor::new(data.clone()), settings)
                    .map_err(|err| {
                        PlaySoundError::IntoSoundError(AudioFileError::FromFileError(err))
                    })?;
                manager
                    .play(static_data)
                    .map_err(play_sound_error_transmute)
                    .map(|mut handle| {
                        if start_paused {
                            error(handle.pause(Tween::default()));
                        }
                        handle
                    })
                    .map(RawAudioHandleImpl::Static)
                    .map(AudioFileHandle)
            }
            Self::Streaming {
                path,
                settings: kira_settings,
            } => {
                let settings = kira_settings
                    .clone()
                    .output_destination(output_destination)
                    .volume(settings.volume)
                    .panning(settings.panning)
                    .loop_region(settings.loop_region)
                    .playback_region(settings.play_region);
                let streaming_sound_data =
                    StreamingSoundData::from_file(path, settings).map_err(|err| {
                        PlaySoundError::IntoSoundError(AudioFileError::FromFileError(err))
                    })?;
                manager
                    .play(streaming_sound_data)
                    .map_err(play_sound_error_cast)
                    .map(|mut handle| {
                        if start_paused {
                            error(handle.pause(Tween::default()));
                        }
                        handle
                    })
                    .map(RawAudioHandleImpl::Streaming)
                    .map(AudioFileHandle)
            }
        }
    }
}

enum RawAudioHandleImpl {
    Static(StaticSoundHandle),
    Streaming(StreamingSoundHandle<FromFileError>),
}

pub struct AudioFileHandle(RawAudioHandleImpl);

macro_rules! defer_call {
    (fn $name:ident(&self $(, $argname:ident: $argtype:ty)*) -> $ret:ty) => {
        defer_call!(fn $name :: $name(&self $(, $argname: $argtype)*) -> $ret);
    };
    // Don't know how to parametrize the `mut` and be able to factor these two into one variant
    (fn $name:ident :: $fnname:ident(&self $(, $argname:ident: $argtype:ty)*) -> $ret:ty) => {
       pub fn $fnname(&self, $($argname: $argtype),*) -> $ret {
            match self {
                Self(RawAudioHandleImpl::Static(handle)) => handle.$name($($argname),*),
                Self(RawAudioHandleImpl::Streaming(handle)) => handle.$name($($argname),*),
            }
        }
    };
    (fn $name:ident(&mut self $(, $argname:ident: $argtype:ty)*) -> $ret:ty) => {
        pub fn $name(&mut self, $($argname: $argtype),*) -> $ret {
            match self {
                Self(RawAudioHandleImpl::Static(handle)) => handle.$name($($argname),*),
                Self(RawAudioHandleImpl::Streaming(handle)) => handle.$name($($argname),*),
            }
        }
    };
}

impl AudioFileHandle {
    defer_call!(fn state :: playback_state(&self) -> PlaybackState);
    defer_call!(fn position(&self) -> f64);
    defer_call!(fn set_playback_rate(&mut self, rate: impl Into<Value<PlaybackRate>>, tween: Tween) -> Result<(), CommandError>);
    defer_call!(fn set_panning(&mut self, panning: impl Into<Value<f64>>, tween: Tween) ->Result<(), CommandError>);
    defer_call!(fn set_playback_region(&mut self, region: impl Into<Region>) -> Result<(), CommandError>);
    defer_call!(fn set_loop_region(&mut self, region: impl Into<Region>) -> Result<(), CommandError>);
    defer_call!(fn pause(&mut self, tween: Tween) -> Result<(), CommandError>);
    defer_call!(fn resume(&mut self, tween: Tween) -> Result<(), CommandError>);
    defer_call!(fn stop(&mut self, tween: Tween) -> Result<(), CommandError>);
    defer_call!(fn seek_to(&mut self, position: f64) -> Result<(), CommandError>);
    defer_call!(fn seek_by(&mut self, amount: f64) -> Result<(), CommandError>);
}
