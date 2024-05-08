use std::fmt;
use std::fmt::Formatter;

pub use ::cpal::*;
use bevy::prelude::*;
use cpal::traits::DeviceTrait;
use kira::manager::backend::cpal::{CpalBackend, CpalBackendSettings};
use kira::manager::backend::mock::{MockBackend, MockBackendSettings};
use kira::manager::backend::{Backend, Renderer};
use thiserror::Error;

/// Allows the user to select an audio backend.
///
/// The default backend uses physical audio devices for output, but there is an alternative "mock" backend that creates
/// a fake output stream, useful for testing.
#[derive(Clone)]
pub enum AudioBackendSelector {
    /// Physical audio backend. Sets up the output stream to use actual audio outputs.
    Physical {
        /// Select a audio device to use to output the audio. `None` lets the system decide which
        /// audio device to use.
        device: Option<cpal::Device>,
        /// Set a specific buffer size for the audio callback coming from the audio device.
        ///
        /// Audio devices do not process audio data sample by sample; they instead process audio
        /// data in chunks. This field controls whether you request a specific size for that chunk.
        /// The audio device is not required to honor that request.
        buffer_size: cpal::BufferSize,
    },
    /// Mock audio backend, used to allow the audio engine to run even when no audio outputs are present on the device.
    ///
    /// This is intended for testing purposes, where manually driving the output stream is required.
    Mock {
        /// This is the target sample rate of the output stream.
        sample_rate: u32,
    },
}

impl fmt::Debug for AudioBackendSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AudioBackendSelector::Physical {
                device,
                buffer_size,
            } => f
                .debug_struct(stringify!(AudioBackendSelector::Physical))
                .field(
                    "device",
                    &device
                        .as_ref()
                        .map(|device| device.name().unwrap_or("Unknown device name".to_string())),
                )
                .field("buffer_size", buffer_size)
                .finish(),
            AudioBackendSelector::Mock { sample_rate } => f
                .debug_struct(stringify!(AudioBackendSelector::Mock))
                .field("sample_rate", sample_rate)
                .finish(),
        }
    }
}

impl Default for AudioBackendSelector {
    fn default() -> Self {
        Self::Physical {
            device: None,
            buffer_size: cpal::BufferSize::Default,
        }
    }
}

/// Enum of possible errors when creating the audio backend.
#[derive(Debug, Error)]
pub enum AudioBackendError {
    /// Error comes from the audio driver
    #[error("Audio driver error: {0}")]
    AudioDriverError(#[from] kira::manager::backend::cpal::Error),
}

/// Audio backend enum.
pub enum AudioBackend {
    /// Physical backend that connects the audio engine to an actual audio output
    Physical(CpalBackend),
    /// Mock backend which provides ways to manually drive the output stream
    Mock(Box<MockBackend>),
}

impl Backend for AudioBackend {
    type Settings = AudioBackendSelector;
    type Error = AudioBackendError;

    fn setup(settings: Self::Settings) -> Result<(Self, u32), Self::Error> {
        match settings {
            AudioBackendSelector::Physical {
                device,
                buffer_size,
            } => {
                let (backend, sample_rate) = CpalBackend::setup(CpalBackendSettings {
                    device,
                    buffer_size,
                })?;
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
