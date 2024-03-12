use crate::AudioWorld;
use bevy::ecs::system::EntityCommand;
use bevy::log::error;
use bevy::prelude::{Entity, World};
use kira::sound::{PlaybackRate, Region};

use kira::tween::{Tween, Value};

macro_rules! impl_command {
    ($name:ident => fn $fnname:ident($($argname:ident: $argtype:ty),*) $(; derive($($derive:ty),*))?) => {
        $(#[derive($($derive),*)])?
        pub struct $name { $(pub $argname: $argtype),* }
        
        impl EntityCommand for $name {
            fn apply(self, id: Entity, world: &mut World) {
                let Self { $($argname),* } = self;
                let mut audio_world = world.resource_mut::<AudioWorld>();
                let Some(handle) = audio_world.audio_handles.get_mut(&id) else {
                    error!("Entity {id:?} does not have any attached audio handles");
                    return;
                };
                if let Err(err) = handle.$fnname($($argname),*) {
                    error!("Cannot process command: {err}");
                }
            }
        }
    };
}

impl_command!(PlayAudio => fn resume(tween: Tween); derive(Default));
impl_command!(PauseAudio => fn pause(tween: Tween); derive(Default));
impl_command!(SetPlaybackRate => fn set_playback_rate(rate: Value<PlaybackRate>, tween: Tween));
impl_command!(SetPanning => fn set_panning(panning: Value<f64>, tween: Tween));
impl_command!(SetPlaybackRegion => fn set_playback_region(region: Region));
impl_command!(SetLoopRegion => fn set_loop_region(region: Region));
impl_command!(SeekTo => fn seek_to(position: f64));
impl_command!(SeekBy => fn seek_by(amount: f64));

impl Default for SetPlaybackRate {
    fn default() -> Self {
        Self {
            rate: Value::Fixed(PlaybackRate::Factor(1.0)),
            tween: Tween::default(),
        }
    }
}

impl Default for SetPanning {
    fn default() -> Self {
        Self {
            panning: Value::Fixed(0.5),
            tween: Tween::default()
        }
    }
}