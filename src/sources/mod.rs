//! Implementations of different audio sources.
use std::fmt;
use std::marker::PhantomData;

use bevy::prelude::*;
use kira::manager::AudioManager;

use crate::{AudioPlaybackSet, AudioSourceSetup, AudioWorld, InternalAudioMarker};
use crate::backend::AudioBackend;
use crate::spatial::SpatialEmitterHandle;

pub mod audio_file;

#[doc(hidden)]
pub mod prelude {
    pub use super::{
        AudioBundle, AudioHandle, AudioSource, AudioSourcePlugin, NoAudioSettings,
        OutputDestination,
    };
    pub use super::audio_file::prelude::*;
}

/// Trait for implementing an audio source to play in the audio engine.
///
/// The audio source needs to provide two things:
///
/// 1. An implementation of [`kira::sound::Sound`] which is going to be sent to the audio engine to
///    generate audio samples
/// 2. A handle which sets up communication between the aforementioned sound and the rest of the
/// world.
///
/// The trait supports a `Settings` struct, which allows users to customize the sound that will
/// be sent before its creation.
pub trait AudioSource: Asset {
    /// Error type that encompasses possible errors that can happen when creating the audio source
    type Error: fmt::Display;
    /// Handle to the audio source, which allows control over the source from a non-audio thread.
    ///
    /// This handle will be stored in a component, which you can get by querying for `AudioHandle<Self::Handle>`.
    type Handle: 'static + Send + Sync;
    /// Settings associated with this audio source, and passed in to the source for its creation.
    type Settings: Send + Sync + Default + Component;

    /// Create an audio handle by calling the manager to play the sound data.
    fn create_handle(
        &self,
        manager: &mut AudioManager<AudioBackend>,
        settings: &Self::Settings,
        output_destination: kira::OutputDestination,
    ) -> Result<Self::Handle, Self::Error>;
}

/// Dummy struct for cases where the audio source has no settings.
#[derive(Debug, Default, Component)]
pub struct NoAudioSettings;

/// Component holding a handle to an [`AudioSource`]. Access this component from your systems to
/// control the parameters of the sound from Bevy.
#[derive(Debug, Deref, DerefMut, Component)]
pub struct AudioHandle<T>(pub T);

/// Audio source plugin, which should be added for each type of [`AudioSource`] you want to use
/// in your game.
#[derive(Debug)]
pub struct AudioSourcePlugin<T>(PhantomData<T>);

impl<T> Default for AudioSourcePlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: AudioSource> Plugin for AudioSourcePlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_asset::<T>().add_systems(
            PostUpdate,
            Self::audio_added
                .in_set(AudioPlaybackSet::Update)
                .in_set(AudioSourceSetup),
        );
    }
}

/// Possible output destinations for the sound. By default, it will be sent directly to the main
/// track, but you can send it to custom tracks with optional processing on them instead.
#[derive(Debug, Default, Component)]
pub enum OutputDestination {
    /// Send the audio data to the main track (default)
    #[default]
    MainOutput,
}

/// [`Bundle`] for easy creation of audio sources.
#[derive(Bundle)]
pub struct AudioBundle<T: AudioSource> {
    /// Handle to the [`AudioSource`] asset to be played.
    pub source: Handle<T>,
    /// Settings related to the sound to play.
    pub settings: T::Settings,
    /// Destination for the audio.
    pub output: OutputDestination,
    /// This is an internal marker for use in internal systems, and needs to be public to be able
    /// to be used properly. You can use it as `With<InternalAudioMarker>` if you want a way to
    /// discriminate entities with audio attached to them.
    pub marker: InternalAudioMarker,
}

impl<T: AudioSource> Default for AudioBundle<T> {
    fn default() -> Self {
        Self {
            source: Handle::default(),
            settings: T::Settings::default(),
            output: OutputDestination::MainOutput,
            marker: InternalAudioMarker,
        }
    }
}

impl<T: AudioSource> AudioSourcePlugin<T> {
    #[allow(clippy::type_complexity)]
    fn audio_added(
        mut commands: Commands,
        mut audio_world: ResMut<AudioWorld>,
        asset_server: Res<AssetServer>,
        assets: Res<Assets<T>>,
        q_added: Query<
            (
                Entity,
                &Handle<T>,
                &T::Settings,
                Option<&SpatialEmitterHandle>,
                &OutputDestination,
            ),
            Without<AudioHandle<T::Handle>>,
        >,
    ) {
        for (entity, source, settings, spatial_emitter, output_destination) in &q_added {
            let output_destination = if let Some(emitter) = spatial_emitter {
                kira::OutputDestination::Emitter(emitter.0.id())
            } else {
                let output_handle = match output_destination {
                    OutputDestination::MainOutput => &*audio_world.audio_manager.main_track(),
                };
                kira::OutputDestination::Track(output_handle.id())
            };
            let result = match assets.get(source) {
                Some(asset)
                    if asset_server.is_loaded_with_dependencies(source)
                        || !asset_server.is_managed(source) =>
                {
                    asset.create_handle(
                        &mut audio_world.audio_manager,
                        settings,
                        output_destination,
                    )
                }
                _ => {
                    debug!("Asset not ready");
                    continue;
                } // Asset not ready, wait
            };
            let handle = match result {
                Ok(handle) => handle,
                Err(err) => {
                    error!("Cannot create handle: {err}");
                    return;
                }
            };
            debug!("Added sound for {} in {entity:?}", T::type_path());
            commands.entity(entity).insert(AudioHandle(handle));
        }
    }
}
