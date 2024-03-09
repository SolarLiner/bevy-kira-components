mod backend;
pub mod commands;
mod diagnostics;
mod loader;
mod manager;
pub mod spatial;
pub mod tracks;

use bevy::ecs::system::{Command, EntityCommand};

use std::io::Cursor;
use std::sync::atomic::{AtomicU64, Ordering};

use bevy::prelude::*;
use kira::manager::backend::{Backend, DefaultBackend};
use kira::manager::AudioManagerSettings;
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings};
use kira::sound::SoundData;
use kira::tween::Tween;

use crate::backend::{AudioBackend, AudioBackendError, AudioBackendSelector};
use crate::loader::AudioLoader;
use crate::manager::{AudioManager, RawAudioHandle};
pub use kira;
use kira::sound::streaming::{StreamingSoundData, StreamingSoundSettings};
use manager::HandleId;
use std::path::PathBuf;
use std::sync::Arc;

use crate::diagnostics::KiraStatisticsDiagnosticPlugin;
use crate::spatial::{SpatialAudioPlugin, SpatialEmitter, SpatialWorld};
use crate::tracks::{AudioTracksPlugin, Track};
pub use loader::AudioLoaderSettings;

pub type AudioSettings = AudioManagerSettings<AudioBackend>;

#[derive(Debug, Default)]
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        let audio_manager_settings = app
            .world
            .remove_non_send_resource::<AudioSettings>()
            .unwrap_or_default();
        app.insert_non_send_resource(
            AudioManager::new(audio_manager_settings).expect("Cannot create audio engine"),
        )
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

#[derive(Component)]
pub struct Audio {
    file: Handle<AudioFile>,
    handle: Option<HandleId>,
    start_paused: bool,
    track_entity: Option<Entity>,
}

impl Audio {
    pub fn new(file: Handle<AudioFile>) -> Self {
        Self {
            file,
            handle: None,
            start_paused: false,
            track_entity: None,
        }
    }

    pub fn start_paused(mut self, start_paused: bool) -> Self {
        self.start_paused = start_paused;
        self
    }

    pub fn in_track(mut self, entity: Entity) -> Self {
        self.track_entity.replace(entity);
        self
    }

    pub fn is_ready(&self) -> bool {
        self.handle.is_some()
    }
}

fn has_audio_to_add(asset_server: Res<AssetServer>, q: Query<&Audio>) -> bool {
    q.into_iter()
        .any(|q| q.handle.is_none() && asset_server.is_loaded_with_dependencies(q.file.id()))
}

fn add_audio(
    mut audio_manager: NonSendMut<AudioManager>,
    spatial_world: Res<SpatialWorld>,
    audio_files: Res<Assets<AudioFile>>,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &mut Audio, Option<&SpatialEmitter>)>,
    q_tracks: Query<&Track>,
) {
    for (entity, mut audio, spatial_emitter) in &mut q {
        let audio = audio.bypass_change_detection();
        if audio.handle.is_some() {
            continue;
        }
        if !asset_server.is_loaded_with_dependencies(audio.file.id()) {
            continue;
        }

        debug!("Audio added to {entity:?}");
        let data = audio_files.get(audio.file.id()).unwrap();
        let result = match data {
            AudioFile::Static(data, settings) => audio_manager
                .kira_manager
                .play(
                    StaticSoundData::from_cursor(Cursor::new(data.clone()), {
                        if spatial_emitter.is_some() {
                            settings
                                .clone()
                                .output_destination(&spatial_world.emitters[&entity])
                        } else {
                            if let Some(track_entity) = audio.track_entity {
                                if let Some(handle) = q_tracks
                                    .get(track_entity)
                                    .ok()
                                    .and_then(|track| track.handle.as_ref())
                                {
                                    settings.clone().output_destination(handle)
                                } else {
                                    settings.clone()
                                }
                            } else {
                                settings.clone()
                            }
                        }
                    })
                    .unwrap(),
                )
                .map(RawAudioHandle::Static)
                .map_err(|err| err.to_string()),
            AudioFile::Streaming { path, settings } => {
                match StreamingSoundData::from_file(path, settings.clone()) {
                    Ok(data) => audio_manager
                        .kira_manager
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
                let id = audio_manager.insert_handle(handle);
                audio.handle.replace(id);
            }
            Err(err) => {
                error!("Cannot play sound: {err}");
            }
        }
    }
}

fn reset_handle_on_audiofile_changed(mut q: Query<(Entity, &mut Audio), Changed<Audio>>) {
    for (entity, mut audio) in &mut q {
        let audio = audio.bypass_change_detection();
        debug!("AudioFile changed on entity {entity:?}, reset handle");
        audio.handle.take();
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
