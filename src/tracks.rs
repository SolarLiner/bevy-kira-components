use bevy::prelude::*;

use crate::AudioWorld;
use kira::track::{TrackBuilder, TrackHandle};

pub struct AudioTracksPlugin;

impl Plugin for AudioTracksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainTrack>()
            .add_systems(PostUpdate, handle_added_tracks);
    }
}

#[derive(Resource)]
pub struct MainTrack(pub TrackHandle);

impl FromWorld for MainTrack {
    fn from_world(world: &mut World) -> Self {
        let audio_world = world.resource::<AudioWorld>();
        let handle = audio_world.audio_manager.main_track();
        Self(handle)
    }
}

#[derive(Component)]
pub struct Track {
    kira_track: Option<TrackBuilder>,
    pub(crate) handle: Option<TrackHandle>,
}

impl Track {
    pub fn new(track: TrackBuilder) -> Self {
        Self {
            kira_track: Some(track),
            handle: None,
        }
    }
}

fn handle_added_tracks(
    mut audio_world: ResMut<AudioWorld>,
    mut q: Query<&mut Track, Added<Track>>,
) {
    for mut track in &mut q {
        let track = track.bypass_change_detection();
        if let Some(track_builder) = track.kira_track.take() {
            match audio_world.audio_manager.add_sub_track(track_builder) {
                Ok(handle) => {
                    track.handle.replace(handle);
                }
                Err(error) => error!("Cannot create track: {error}"),
            }
        }
    }
}

fn handle_removed_tracks(
    mut audio_world: ResMut<AudioWorld>,
    mut removed: RemovedComponents<Track>,
) {
    for entity in removed.read() {
        audio_world.tracks.remove(&entity);
    }
}

#[derive(Deref, DerefMut, Component)]
pub struct EffectHandle<E>(pub E);
