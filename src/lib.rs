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
mod backend;
pub mod commands;
mod diagnostics;
mod loader;
pub mod spatial;
pub mod tracks;

use std::io::Cursor;

use bevy::prelude::*;

use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings};

use kira::tween::{Tween, Value};

use crate::backend::AudioBackend;
use crate::loader::AudioLoader;
pub use kira;
use kira::sound::streaming::{StreamingSoundData, StreamingSoundHandle, StreamingSoundSettings};
use kira::sound::{FromFileError, PlaybackRate, PlaybackState, Region};

use bevy::ecs::entity::EntityHashMap;
use kira::CommandError;
use std::path::PathBuf;
use std::sync::Arc;

use crate::diagnostics::KiraStatisticsDiagnosticPlugin;
use crate::spatial::{SpatialAudioPlugin, SpatialEmitter, SpatialWorld};
use crate::tracks::{AudioTracksPlugin, Track};
pub use loader::AudioLoaderSettings;

/// Type of settings for the audio engine. Insert it as a resource before adding the plugin to
/// change the default settings.
pub type AudioSettings = AudioManagerSettings<AudioBackend>;

/// Adds audio to Bevy games via the [`kira`] crate.
#[derive(Debug, Default)]
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioWorld>()
            .init_asset::<AudioFile>()
            .init_asset_loader::<AudioLoader>()
            .add_plugins((
                AudioTracksPlugin,
                SpatialAudioPlugin,
                KiraStatisticsDiagnosticPlugin,
            ))
            .add_systems(
                PreUpdate,
                (
                    reset_handle_on_audiofile_changed,
                    add_audio.run_if(has_audio_to_add),
                )
                    .chain(),
            )
            .add_systems(Last, remove_audio);
    }
}

/// Main resource holding all the bookkeeping necessary to link the ECS data to the audio engine.
/// As a user, you'll want to use this to query information on the current audio engine settings, as
/// well as properties on audio entities.
#[derive(Resource)]
pub struct AudioWorld {
    pub(crate) audio_manager: AudioManager<AudioBackend>,
    pub(crate) audio_handles: EntityHashMap<RawAudioHandle>,
}

impl FromWorld for AudioWorld {
    fn from_world(world: &mut World) -> Self {
        let audio_manager_settings = world
            .remove_non_send_resource::<AudioSettings>()
            .unwrap_or_default();
        let audio_manager =
            AudioManager::new(audio_manager_settings).expect("Cannot create audio backend");
        Self {
            audio_manager,
            audio_handles: EntityHashMap::default(),
        }
    }
}

macro_rules! defer_handle_call {
    // Don't know how to parametrize the `mut` and be able to factor these two into one variant
    (fn $name:ident(&self $(, $argname:ident: $argtype:ty)*) -> $ret:ty) => {
        pub fn $name(&self, entity: Entity, $($argname: $argtype),*) -> Option<$ret> {
            let handle = self.audio_handles.get(&entity)?;
            Some(handle.$name($($argname),*))
        }
    };
    (fn $name:ident(&mut self $(, $argname:ident: $argtype:ty)*) -> $ret:ty) => {
        pub fn $name(&mut self, entity: Entity, $($argname: $argtype),*) -> Option<$ret> {
            let handle = self.audio_handles.get_mut(&entity)?;
            Some(handle.$name($($argname),*))
        }
    };
}

impl AudioWorld {
    defer_handle_call!(fn state(&self) -> PlaybackState);
    defer_handle_call!(fn position(&self) -> f64);
}

/// Tag the entity as being an audio source. Use [`Audio::new`] to create the component with a
/// [`Handle`] to an [`AudioFile`].
#[derive(Component, Clone)]
pub struct Audio {
    /// Audio file this component is playing. You can change the handle to trigger a restart with
    /// a new audio file.
    pub file: Handle<AudioFile>,
    start_paused: bool,
}

impl Audio {
    /// Create a new audio source component, playing this [`AudioFile`]. The ECS system will
    /// automatically add it to the audio engine and start playing it.
    pub fn new(file: Handle<AudioFile>) -> Self {
        Self {
            file,
            start_paused: false,
        }
    }

    /// Set whether the audio source starts paused, or if it starts playing right away (the
    /// default).
    pub fn start_paused(mut self, start_paused: bool) -> Self {
        self.start_paused = start_paused;
        self
    }
}

/// Mark this entity as playing through the specified track.
///
/// This only affects the spawning of the audio source entity, as [`kira`] does not allow
/// re-routing of audio signals after creation. If you do wish to do so, you'll need to despawn
/// the entity and create a new one with this component attached directly.
#[derive(Component, Copy, Clone)]
pub struct AudioTrack(pub Entity);

fn has_audio_to_add(asset_server: Res<AssetServer>, q: Query<&Audio>) -> bool {
    q.into_iter()
        .any(|q| asset_server.is_loaded_with_dependencies(q.file.id()))
}

fn add_audio(
    mut audio_world: ResMut<AudioWorld>,
    spatial_world: Res<SpatialWorld>,
    audio_files: Res<Assets<AudioFile>>,
    asset_server: Res<AssetServer>,
    mut q: Query<(
        Entity,
        &mut Audio,
        Option<&SpatialEmitter>,
        Option<&AudioTrack>,
    )>,
    q_tracks: Query<&Track>,
) {
    for (entity, mut audio, spatial_emitter, audio_track) in &mut q {
        let audio = audio.bypass_change_detection();
        if audio_world.audio_handles.contains_key(&entity)
            || !asset_server.is_loaded_with_dependencies(audio.file.id())
        {
            continue;
        }

        debug!("Audio added to {entity:?}");
        let data = audio_files.get(audio.file.id()).unwrap();
        let result = match data {
            AudioFile::Static(data, settings) => audio_world
                .audio_manager
                .play(
                    StaticSoundData::from_cursor(Cursor::new(data.clone()), {
                        if spatial_emitter.is_some() && spatial_world.emitters.contains_key(&entity)
                        {
                            (*settings).output_destination(&spatial_world.emitters[&entity])
                        } else if let Some(AudioTrack(track_entity)) = audio_track.copied() {
                            if let Some(handle) = q_tracks
                                .get(track_entity)
                                .ok()
                                .and_then(|track| track.handle.as_ref())
                            {
                                (*settings).output_destination(handle)
                            } else {
                                *settings
                            }
                        } else {
                            *settings
                        }
                    })
                    .unwrap(),
                )
                .map(RawAudioHandle::Static)
                .map_err(|err| err.to_string()),
            AudioFile::Streaming { path, settings } => {
                match StreamingSoundData::from_file(path, {
                    if spatial_emitter.is_some() && spatial_world.emitters.contains_key(&entity) {
                        (*settings).output_destination(&spatial_world.emitters[&entity])
                    } else if let Some(AudioTrack(track_entity)) = audio_track.copied() {
                        if let Some(handle) = q_tracks
                            .get(track_entity)
                            .ok()
                            .and_then(|track| track.handle.as_ref())
                        {
                            (*settings).output_destination(handle)
                        } else {
                            *settings
                        }
                    } else {
                        *settings
                    }
                }) {
                    Ok(data) => audio_world
                        .audio_manager
                        .play(data)
                        .map(RawAudioHandle::Streaming)
                        .map_err(|err| err.to_string()),
                    Err(error) => {
                        error!("Cannot play {}: {error}", path.display());
                        continue;
                    }
                }
            }
        };
        match result {
            Ok(mut handle) => {
                if audio.start_paused {
                    handle.pause(Tween::default()).unwrap();
                }
                audio_world.audio_handles.insert(entity, handle);
            }
            Err(err) => {
                error!("Cannot play sound: {err}");
            }
        }
    }
}

fn remove_audio(mut audio_world: ResMut<AudioWorld>, mut removed: RemovedComponents<Audio>) {
    for entity in removed.read() {
        info!("Audio removed on {entity:?}");
        audio_world.audio_handles.remove(&entity);
    }
}

fn reset_handle_on_audiofile_changed(
    mut audio_world: ResMut<AudioWorld>,
    mut q: Query<Entity, Changed<Audio>>,
) {
    for entity in &mut q {
        debug!("AudioFile changed on entity {entity:?}, reset handle");
        audio_world.audio_handles.remove(&entity);
    }
}

/// Bevy [`Asset`] implementation that wraps audio data for [`kira`].
///
/// Streaming audio data is currently not possible over the internet, so when targeting the web,
/// all audio sources need to be [`Static`](Self::Static).
#[derive(Asset, TypePath)]
pub enum AudioFile {
    Static(Arc<[u8]>, StaticSoundSettings),
    Streaming {
        path: PathBuf,
        settings: StreamingSoundSettings,
    },
}

pub(crate) enum RawAudioHandle {
    Static(StaticSoundHandle),
    Streaming(StreamingSoundHandle<FromFileError>),
}

macro_rules! defer_call {
    // Don't know how to parametrize the `mut` and be able to factor these two into one variant
    (fn $name:ident(&self $(, $argname:ident: $argtype:ty)*) -> $ret:ty) => {
        pub(crate) fn $name(&self, $($argname: $argtype),*) -> $ret {
            match self {
                Self::Static(handle) => handle.$name($($argname),*),
                Self::Streaming(handle) => handle.$name($($argname),*),
            }
        }
    };
    (fn $name:ident(&mut self $(, $argname:ident: $argtype:ty)*) -> $ret:ty) => {
        pub(crate) fn $name(&mut self, $($argname: $argtype),*) -> $ret {
            match self {
                Self::Static(handle) => handle.$name($($argname),*),
                Self::Streaming(handle) => handle.$name($($argname),*),
            }
        }
    };
}

impl RawAudioHandle {
    defer_call!(fn state(&self) -> PlaybackState);
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
