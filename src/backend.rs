use bevy::prelude::*;
use kira::manager::backend::cpal::CpalBackend;
use kira::manager::backend::mock::{MockBackend, MockBackendSettings};
use kira::manager::backend::{cpal, Backend, Renderer};
use thiserror::Error;

/// Allows the user to select an audio backend.
///
/// The default backend uses physical audio devices for output, but there is an alternative "mock" backend that creates
/// a fake output stream, useful for testing.
#[derive(Debug, Copy, Clone, Default)]
pub enum AudioBackendSelector {
    /// Physical audio backend. Sets up the output stream to use actual audio outputs.
    #[default]
    Physical,
    /// Mock audio backend, used to allow the audio engine to run even when no audio outputs are present on the device.
    ///
    /// This is intended for testing purposes, where manually driving the output stream is required.
    Mock {
        /// This is the target sample rate of the output stream.
        sample_rate: u32,
    },
}

/// Enum of possible errors when creating the audio backend.
#[derive(Debug, Error)]
pub enum AudioBackendError {
    /// Error comes from the audio driver
    #[error("Audio driver error: {0}")]
    AudioDriverError(#[from] cpal::Error),
}

/// Audio backend enum.
pub enum AudioBackend {
    /// Physical backend that connects the audio engine to an actual audio output
    Physical(CpalBackend),
    // Mock backend which provides ways to manually drive the output stream
    Mock(Box<MockBackend>),
}

impl Backend for AudioBackend {
    type Settings = AudioBackendSelector;
    type Error = AudioBackendError;

    fn setup(settings: Self::Settings) -> Result<(Self, u32), Self::Error> {
        match settings {
            AudioBackendSelector::Physical => {
                let (backend, sample_rate) = CpalBackend::setup(())?;
                Ok((Self::Physical(backend), sample_rate))
            }
            AudioBackendSelector::Mock { sample_rate } => {
                let (backend, sample_rate) =
                    MockBackend::setup(MockBackendSettings { sample_rate }).unwrap();
                Ok((Self::Mock(Box::new(backend)), sample_rate))
            }
        }
    }

    fn start(&mut self, renderer: Renderer) -> Result<(), Self::Error> {
        match self {
            Self::Physical(backend) => backend.start(renderer).map_err(Into::into),
            Self::Mock(mock) => {
                mock.start(renderer).unwrap();
                Ok(())
            }
        }
    }
}
