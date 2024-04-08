//! Implementation of tracks support.
use bevy::prelude::*;

use crate::{AudioPlaybackSet, AudioWorld};

use kira::track::TrackBuilder;

#[doc(hidden)]
#[allow(missing_docs)]
pub mod prelude {
    pub use super::{EffectHandle, MainTrack, OutputTrack, Track};
}

/// Plugin for adding support to using tracks with Kira in Bevy.
pub(crate) struct AudioTracksPlugin;

impl Plugin for AudioTracksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainTrack>().add_systems(
            PostUpdate,
            handle_added_tracks.in_set(AudioPlaybackSet::Update),
        );
    }
}

/// Resource holding the [`TrackHandle`] to the main track.
#[derive(Resource)]
pub struct MainTrack(pub kira::track::TrackHandle);

impl FromWorld for MainTrack {
    fn from_world(world: &mut World) -> Self {
        let audio_world = world.resource::<AudioWorld>();
        let handle = audio_world.audio_manager.main_track();
        Self(handle)
    }
}

/// Component marking this entity as being a track. Provide a [`TrackBuilder`], which are the
/// settings applied to this new track.
#[derive(Default, Component)]
pub struct Track(pub TrackBuilder);

/// Handle to the track in the audio engine. Access this component to make changes to the track
/// in your systems.
#[derive(Component, Deref, DerefMut)]
pub struct TrackHandle(pub(crate) kira::track::TrackHandle);

fn handle_added_tracks(
    mut commands: Commands,
    mut audio_world: ResMut<AudioWorld>,
    mut q: Query<(Entity, &mut Track), Added<Track>>,
) {
    for (entity, mut track) in &mut q {
        let track = track.bypass_change_detection();
        let track_builder = std::mem::take(&mut track.0);
        match audio_world.audio_manager.add_sub_track(track_builder) {
            Ok(handle) => {
                commands.entity(entity).insert(TrackHandle(handle));
            }
            Err(error) => error!("Cannot create track: {error}"),
        }
    }
}

/// Effect handle component. Add the handle to an effect you've added to a track to be able to
/// control it from here.
#[derive(Deref, DerefMut, Component)]
pub struct EffectHandle<E>(pub E);

/// Mark this entity as playing through the specified track.
///
/// This only affects the spawning of the audio source entity, as [`kira`] does not allow
/// re-routing of audio signals after creation. If you do wish to do so, you'll need to despawn
/// the entity and create a new one with this component attached directly.
#[derive(Component, Copy, Clone)]
pub struct OutputTrack(pub Entity);
