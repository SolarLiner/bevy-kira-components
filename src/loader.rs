use crate::AudioFile;
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::utils::BoxedFuture;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::sound::streaming::{StreamingSoundData, StreamingSoundSettings};
use kira::sound::{FromFileError, Region};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::ops::RangeFull;
use thiserror::Error;

#[derive(Default)]
pub struct AudioLoader;

/// Possible errors that can be produced by [`AudioLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AudioLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// An Error loading sound from a file. See [`FromFileError`]
    #[error("Error while loading a sound: {0}")]
    FileError(#[from] FromFileError),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AudioLoaderSettings {
    pub looping: bool,
    streaming: bool,
}

impl Default for AudioLoaderSettings {
    fn default() -> Self {
        Self {
            looping: false,
            streaming: false,
        }
    }
}

impl AssetLoader for AudioLoader {
    type Asset = AudioFile;
    type Settings = AudioLoaderSettings;
    type Error = AudioLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a AudioLoaderSettings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            if _settings.streaming {
                Ok(AudioFile::Streaming {
                    path: _load_context.path().to_path_buf(),
                    settings: StreamingSoundSettings::new()
                        .loop_region(_settings.looping.then(|| RangeFull.into())),
                })
            } else {
                let mut sound_bytes = vec![];
                reader.read_to_end(&mut sound_bytes).await?;
                Ok(AudioFile::Static(
                    sound_bytes.into(),
                    StaticSoundSettings::default()
                        .loop_region(_settings.looping.then(|| RangeFull.into())),
                ))
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wav", "flac", "mp3", "ogg", "oga", "spx"]
    }
}
