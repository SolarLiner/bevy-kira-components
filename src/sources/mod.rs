pub mod audio_file;

use crate::backend::AudioBackend;
use crate::tracks::Track;
use crate::{AudioPlaybackSet, AudioSourceSetup, AudioWorld, InternalAudioMarker};
use bevy::prelude::*;
use kira::manager::AudioManager;
use kira::sound::{PlaybackRate, PlaybackState, Region, SoundData};
use kira::spatial::emitter::{EmitterHandle, EmitterSettings};
use kira::track::TrackHandle;
use kira::tween::{Tween, Value};
use kira::CommandError;
use std::fmt;
use std::marker::PhantomData;
use crate::spatial::SpatialEmitterHandle;

#[doc(hidden)]
pub mod prelude {
    pub use super::audio_file::prelude::*;
    pub use super::{AudioBundle, AudioHandle, AudioSource, AudioSourcePlugin, OutputDestination};
}

pub trait AudioSource: Asset {
    type Error: fmt::Display;
    type Handle: 'static + Send + Sync;
    type Settings: Send + Sync + Default + Component;

    fn create_handle(
        &self,
        manager: &mut AudioManager<AudioBackend>,
        settings: &Self::Settings,
        output_destination: kira::OutputDestination,
    ) -> Result<Self::Handle, Self::Error>;
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AudioHandle<T>(pub T);

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

#[derive(Debug, Default, Component)]
pub enum OutputDestination {
    #[default]
    MainOutput,
    SpecificTrack(Entity),
}

#[derive(Bundle)]
pub struct AudioBundle<T: AudioSource> {
    pub source: Handle<T>,
    pub settings: T::Settings,
    pub output: OutputDestination,
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
    fn audio_added(
        mut commands: Commands,
        mut audio_world: ResMut<AudioWorld>,
        assets: Res<Assets<T>>,
        q_added: Query<
            (Entity, &Handle<T>, &T::Settings, Option<&SpatialEmitterHandle>, &OutputDestination),
            Without<AudioHandle<T::Handle>>,
        >,
        q_track: Query<&crate::tracks::TrackHandle>,
    ) {
        let main_track_handle = audio_world.audio_manager.main_track();
        for (entity, source, settings, spatial_emitter, output_destination) in &q_added {
            let output_destination = if let Some(emitter) = spatial_emitter {
                kira::OutputDestination::Emitter(emitter.0.id())
            } else {
                let output_handle = match output_destination {
                    OutputDestination::MainOutput => &main_track_handle,
                    OutputDestination::SpecificTrack(entity) => match q_track.get(*entity) {
                        Ok(crate::tracks::TrackHandle(handle)) => handle,
                        _ => {
                            error!(
                            "Entity {entity:?} does not have a Track component, sending \
                            audio to main output instead"
                        );
                            &main_track_handle
                        }
                    },
                };
                kira::OutputDestination::Track(output_handle.id())
            };
            let result = assets.get(source).unwrap().create_handle(
                &mut audio_world.audio_manager,
                settings,
                output_destination,
            );
            let handle = match result {
                Ok(handle) => handle,
                Err(err) => {
                    error!("Cannot create handle: {err}");
                    return;
                }
            };
            commands.entity(entity).insert(AudioHandle(handle));
        }
    }
}
