use crate::AudioWorld;
use bevy::ecs::system::EntityCommand;
use bevy::log::error;
use bevy::prelude::{Entity, World};
use kira::sound::PlaybackRate;

use kira::tween::{Tween, Value};

#[derive(Default)]
pub struct PlayAudio(pub Tween);

impl EntityCommand for PlayAudio {
    fn apply(self, entity: Entity, world: &mut World) {
        let mut audio_world = world.resource_mut::<AudioWorld>();
        let Some(handle) = audio_world.audio_handles.get_mut(&entity) else {
            error!("Entity {entity:?} does not have any attached audio handles");
            return;
        };
        if let Err(err) = handle.resume(self.0) {
            error!("Cannot process play command: {err}");
        }
    }
}

#[derive(Default)]
pub struct PauseAudio(pub Tween);

impl EntityCommand for PauseAudio {
    fn apply(self, entity: Entity, world: &mut World) {
        let mut audio_world = world.resource_mut::<AudioWorld>();
        let Some(handle) = audio_world.audio_handles.get_mut(&entity) else {
            error!("Entity {entity:?} does not have any attached audio handles");
            return;
        };
        if let Err(err) = handle.pause(self.0) {
            error!("Cannot process pause command: {err}");
        }
    }
}

pub struct SetPlaybackSpeed(pub Value<PlaybackRate>, pub Tween);

impl EntityCommand for SetPlaybackSpeed {
    fn apply(self, id: Entity, world: &mut World) {
        let mut audio_world = world.resource_mut::<AudioWorld>();
        let Some(handle) = audio_world.audio_handles.get_mut(&id) else {
            error!("Entity {id:?} does not have any attached audio handles");
            return;
        };
        if let Err(err) = handle.set_playback_rate(self.0, self.1) {
            error!("Cannot process set playback speed command: {err}")
        }
    }
}
