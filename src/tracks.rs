use crate::manager::AudioManager;
use bevy::prelude::*;
use kira::spatial::scene::SpatialSceneSettings;
use kira::track::effect::EffectBuilder;
use kira::track::{TrackBuilder, TrackHandle};

pub struct AudioTracksPlugin;

impl Plugin for AudioTracksPlugin {
    fn build(&self, app: &mut App) {
        let track_handle = app
            .world
            .non_send_resource::<AudioManager>()
            .kira_manager
            .main_track();
        app.insert_resource(MainTrack(track_handle));
        app.add_systems(PostUpdate, handle_changed_tracks);
    }
}

#[derive(Resource)]
pub struct MainTrack(pub TrackHandle);

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

fn handle_changed_tracks(
    mut audio_manager: NonSendMut<AudioManager>,
    mut q: Query<&mut Track, Added<Track>>,
) {
    for mut track in &mut q {
        let track = track.bypass_change_detection();
        if let Some(track_builder) = track.kira_track.take() {
            match audio_manager.kira_manager.add_sub_track(track_builder) {
                Ok(handle) => {
                    track.handle.replace(handle);
                }
                Err(error) => error!("Cannot create track: {error}"),
            }
        }
    }
}

#[derive(Deref, DerefMut, Component)]
pub struct EffectHandle<E>(pub E);
