use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

use bevy::prelude::*;
use kira::manager::AudioManagerSettings;
use kira::sound::static_sound::StaticSoundHandle;
use kira::sound::streaming::StreamingSoundHandle;
use kira::sound::FromFileError;
use kira::tween::Tween;

use crate::backend::{AudioBackend, AudioBackendError};

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

pub struct AudioManager {
    pub(crate) kira_manager: kira::manager::AudioManager<AudioBackend>,
    raw_handles: BTreeMap<HandleId, RawAudioHandle>,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new(default()).unwrap()
    }
}

impl AudioManager {
    pub fn new(settings: AudioManagerSettings<AudioBackend>) -> Result<Self, AudioBackendError> {
        Ok(Self {
            kira_manager: kira::manager::AudioManager::new(settings)?,
            raw_handles: BTreeMap::new(),
        })
    }

    pub(crate) fn insert_handle(&mut self, raw_audio_handle: RawAudioHandle) -> HandleId {
        let id = HandleId::new();
        self.raw_handles.insert(id, raw_audio_handle);
        id
    }

    pub(crate) fn get_raw_handle_mut(
        &mut self,
        handle_id: HandleId,
    ) -> Option<&mut RawAudioHandle> {
        self.raw_handles.get_mut(&handle_id)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct HandleId(u64);

impl HandleId {
    fn new() -> Self {
        static NEXT_HANDLE_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_HANDLE_ID.fetch_add(1, Ordering::SeqCst))
    }
}
