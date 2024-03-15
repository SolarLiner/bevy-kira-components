use bevy::prelude::*;
use kira::manager::backend::cpal::CpalBackend;
use kira::manager::backend::mock::{MockBackend, MockBackendSettings};
use kira::manager::backend::{cpal, Backend, Renderer};
use thiserror::Error;

#[derive(Debug, Copy, Clone, Default)]
pub enum AudioBackendSelector {
    #[default]
    Physical,
    Mock {
        sample_rate: u32,
    },
}

#[derive(Debug, Error)]
pub enum AudioBackendError {
    #[error("Audio driver error: {0}")]
    AudioDriverError(#[from] cpal::Error),
}

pub enum AudioBackend {
    Physical(CpalBackend),
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
