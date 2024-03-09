use crate::manager::AudioManager;
use crate::Audio;
use bevy::ecs::system::EntityCommand;
use bevy::log::error;
use bevy::prelude::{Entity, World};

use kira::tween::Tween;


pub struct PlayAudio(pub Tween);

impl EntityCommand for PlayAudio {
    fn apply(self, entity: Entity, world: &mut World) {
        let Some(audio) = world.entity(entity).get::<Audio>() else {
            error!("Cannot play audio on entity without audio component");
            return;
        };
        let Some(handle) = audio.handle else {
            error!("Entity isn't known to the audio manager");
            return;
        };
        let mut audio_manager = world.get_non_send_resource_mut::<AudioManager>().unwrap();
        let handle = audio_manager.get_raw_handle_mut(handle).unwrap();
        if let Err(err) = handle.resume(self.0) {
            error!("Cannot process play command: {err}");
        }
    }
}

pub struct PauseAudio(pub Tween);

impl EntityCommand for PauseAudio {
    fn apply(self, entity: Entity, world: &mut World) {
        let Some(audio) = world.entity(entity).get::<Audio>() else {
            error!("Cannot play audio on entity without audio component");
            return;
        };
        let Some(handle) = audio.handle else {
            error!("Entity isn't known to the audio manager");
            return;
        };
        let mut audio_manager = world.get_non_send_resource_mut::<AudioManager>().unwrap();
        let handle = audio_manager.get_raw_handle_mut(handle).unwrap();
        if let Err(err) = handle.pause(self.0) {
            error!("Cannot process pause command: {err}");
        }
    }
}
