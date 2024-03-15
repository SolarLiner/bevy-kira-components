use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::utils::BoxedFuture;
use kira::sound::static_sound::StaticSoundSettings;
use kira::sound::streaming::StreamingSoundSettings;
use kira::sound::FromFileError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::sources::audio_file::AudioFile;

#[derive(Default)]
pub struct AudioFileLoader;

/// Possible errors that can be produced by [`AudioFileLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AudioFileLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// An Error loading sound from a file. See [`FromFileError`]
    #[error("Error while loading a sound: {0}")]
    FileError(#[from] FromFileError),
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct AudioAssetSettings {
    pub should_stream: bool,
}

impl Default for AudioAssetSettings {
    fn default() -> Self {
        Self {
            should_stream: false,
        }
    }
}

impl AssetLoader for AudioFileLoader {
    type Asset = AudioFile;
    type Settings = AudioAssetSettings;
    type Error = AudioFileLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        settings: &'a AudioAssetSettings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            if settings.should_stream {
                Ok(AudioFile::Streaming {
                    path: load_context.path().to_path_buf(),
                    settings: StreamingSoundSettings::new(),
                })
            } else {
                let mut sound_bytes = vec![];
                reader.read_to_end(&mut sound_bytes).await?;
                Ok(AudioFile::Static(
                    sound_bytes.into(),
                    StaticSoundSettings::default(),
                ))
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wav", "flac", "mp3", "ogg", "oga", "spx"]
    }
}
