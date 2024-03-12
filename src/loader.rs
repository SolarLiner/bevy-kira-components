use crate::AudioFile;
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::utils::BoxedFuture;
use kira::sound::static_sound::StaticSoundSettings;
use kira::sound::streaming::StreamingSoundSettings;
use kira::sound::{FromFileError, Region};
use serde::{Deserialize, Serialize};

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

/// Settings used when loading an audio file.
/// 
/// By default, audio files are loaded with their entire contents in memory. This allows fast 
/// seeking and looping, but needs more memory. If the files are too big, and go beyond your 
/// memory budget, use the [`Self::streaming`] option instead.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct AudioLoaderSettings {
    /// When set to true, the file is not directly loaded, but instead chunks of the file are 
    /// loaded at a time when needed by the audio engine.
    /// Note that this requires disk access directly in [`kira`], and therefore cannot be used in
    /// the web.
    pub streaming: bool,
    /// Optional looping region. When set, the playback will start as normal, and when the 
    /// playback arrives at the end of the region,it will jump back at the set beginning of the 
    /// looping region. The sound will therefore never stop playing automatically.
    pub looping: Option<Region>,
    // TODO: Implement more settings
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
                    settings: StreamingSoundSettings::new().loop_region(_settings.looping),
                })
            } else {
                let mut sound_bytes = vec![];
                reader.read_to_end(&mut sound_bytes).await?;
                Ok(AudioFile::Static(
                    sound_bytes.into(),
                    StaticSoundSettings::default().loop_region(_settings.looping),
                ))
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["wav", "flac", "mp3", "ogg", "oga", "spx"]
    }
}
