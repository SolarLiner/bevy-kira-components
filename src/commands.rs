use crate::AudioWorld;
use bevy::ecs::system::EntityCommand;
use bevy::log::error;
use bevy::prelude::{Entity, World};

use kira::tween::Tween;

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
