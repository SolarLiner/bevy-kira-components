mod backend;
pub mod commands;
mod diagnostics;
mod loader;
pub mod spatial;
pub mod tracks;


use std::collections::BTreeMap;
use std::io::Cursor;


use bevy::prelude::*;

use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings};

use kira::tween::Tween;

use crate::backend::AudioBackend;
use crate::loader::AudioLoader;
pub use kira;
use kira::sound::streaming::{StreamingSoundData, StreamingSoundHandle, StreamingSoundSettings};
use std::path::PathBuf;
use std::sync::Arc;
use kira::track::TrackHandle;
use kira::sound::FromFileError;

use crate::diagnostics::KiraStatisticsDiagnosticPlugin;
use crate::spatial::{SpatialAudioPlugin, SpatialEmitter, SpatialWorld};
use crate::tracks::{AudioTracksPlugin, Track};
pub use loader::AudioLoaderSettings;

pub type AudioSettings = AudioManagerSettings<AudioBackend>;

#[derive(Debug, Default)]
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AudioWorld>()
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
        );
    }
}

#[derive(Resource)]
pub(crate) struct AudioWorld {
    pub(crate) audio_manager: AudioManager<AudioBackend>,
    pub(crate) audio_handles: BTreeMap<Entity, RawAudioHandle>,
    pub(crate) tracks: BTreeMap<Entity, TrackHandle>,
}

impl FromWorld for AudioWorld {
    fn from_world(world: &mut World) -> Self {
        let audio_manager_settings = 
            world
            .remove_non_send_resource::<AudioSettings>()
            .unwrap_or_default();
        let audio_manager = AudioManager::new(audio_manager_settings).expect("Cannot create audio backend");
        Self {
            audio_manager,
            audio_handles: BTreeMap::new(),
            tracks: BTreeMap::new(),
        }
    }
}

#[derive(Component, Clone)]
pub struct Audio {
    file: Handle<AudioFile>,
    start_paused: bool,
}

#[derive(Component, Copy, Clone)]
pub struct AudioTrack(pub Entity);

impl Audio {
    pub fn new(file: Handle<AudioFile>) -> Self {
        Self {
            file,
            start_paused: false,
        }
    }

    pub fn start_paused(mut self, start_paused: bool) -> Self {
        self.start_paused = start_paused;
        self
    }
}

fn has_audio_to_add(asset_server: Res<AssetServer>, q: Query<&Audio>) -> bool {
    q.into_iter()
        .any(|q| asset_server.is_loaded_with_dependencies(q.file.id()))
}

fn add_audio(
    mut audio_world: ResMut<AudioWorld>,
    spatial_world: Res<SpatialWorld>,
    audio_files: Res<Assets<AudioFile>>,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &mut Audio, Option<&SpatialEmitter>, Option<&AudioTrack>)>,
    q_tracks: Query<&Track>,
) {
    for (entity, mut audio, spatial_emitter, audio_track) in &mut q {
        let audio = audio.bypass_change_detection();
        if audio_world.audio_handles.contains_key(&entity) || !asset_server.is_loaded_with_dependencies(audio.file.id()) {
            continue;
        }

        debug!("Audio added to {entity:?}");
        let data = audio_files.get(audio.file.id()).unwrap();
        let result = match data {
            AudioFile::Static(data, settings) => audio_world
                .audio_manager
                .play(
                    StaticSoundData::from_cursor(Cursor::new(data.clone()), {
                        if spatial_emitter.is_some() {
                            (*settings)
                                .output_destination(&spatial_world.emitters[&entity])
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
                    if spatial_emitter.is_some() {
                        (*settings)
                            .output_destination(&spatial_world.emitters[&entity])
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

fn remove_audio(
    mut audio_world: ResMut<AudioWorld>,
    mut removed: RemovedComponents<Audio>,
) {
    for entity in removed.read() {
        audio_world.audio_handles.remove(&entity);
    }
}

fn reset_handle_on_audiofile_changed(mut audio_world: ResMut<AudioWorld>, mut q: Query<Entity, Changed<Audio>>) {
    for entity in &mut q {
        debug!("AudioFile changed on entity {entity:?}, reset handle");
        audio_world.audio_handles.remove(&entity);
    }
}

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

impl RawAudioHandle {
    pub(crate) fn resume(&mut self, tween: Tween) -> Result<(), kira::CommandError> {
        match self {
            Self::Static(handle) => handle.resume(tween),
            Self::Streaming(handle) => handle.resume(tween),
        }
    }

    pub(crate) fn pause(&mut self, tween: Tween) -> Result<(), kira::CommandError> {
        match self {
            Self::Static(handle) => handle.pause(tween),
            Self::Streaming(handle) => handle.pause(tween),
        }
    }
}
