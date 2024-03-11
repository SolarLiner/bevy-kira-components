use crate::{add_audio, Audio, AudioWorld};

use bevy::prelude::*;

use kira::spatial::emitter::EmitterHandle;
use kira::spatial::listener::ListenerHandle;
use kira::spatial::scene::{SpatialSceneHandle, SpatialSceneSettings};
use kira::tween::Tween;

use std::collections::BTreeMap;
use bevy::ecs::entity::EntityHashMap;

pub struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialWorld>()
            .add_systems(PreUpdate, (add_listeners, add_emitters.before(add_audio)))
            .add_systems(PostUpdate, (update_listeners, update_emitters));
    }
}

#[derive(Component)]
pub struct AudioListener;

#[derive(Component)]
pub struct SpatialEmitter;

#[derive(Resource)]
pub struct SpatialWorld {
    pub(crate) spatial_handle: SpatialSceneHandle,
    pub(crate) emitters: EntityHashMap<EmitterHandle>,
    pub(crate) listeners: EntityHashMap<ListenerHandle>,
}

impl FromWorld for SpatialWorld {
    fn from_world(world: &mut World) -> Self {
        let settings = world
            .remove_non_send_resource::<SpatialSceneSettings>()
            .unwrap_or_default();
        let mut audio_world = world.resource_mut::<AudioWorld>();
        let spatial_handle = audio_world
            .audio_manager
            .add_spatial_scene(settings)
            .expect("Cannot create audio spatial world");
        Self {
            spatial_handle,
            emitters: EntityHashMap::default(),
            listeners: EntityHashMap::default(),
        }
    }
}

fn add_listeners(
    mut spatial_world: ResMut<SpatialWorld>,
    q: Query<(Entity, &GlobalTransform), Added<AudioListener>>,
) {
    for (entity, global_transform) in &q {
        let (_, quat, position) = global_transform.to_scale_rotation_translation();
        let listener = spatial_world
            .spatial_handle
            .add_listener(position, quat, default())
            .unwrap();
        debug!("Add listener to {entity:?}");
        spatial_world.listeners.insert(entity, listener);
    }
}

fn add_emitters(
    mut spatial_world: ResMut<SpatialWorld>,
    q: Query<
        (Entity, &GlobalTransform),
        (With<Audio>, With<SpatialEmitter>, Added<GlobalTransform>),
    >,
) {
    for (entity, global_transform) in &q {
        let result = spatial_world
            .spatial_handle
            .add_emitter(global_transform.translation(), default());
        debug!("Add emitter to {entity:?}");
        match result {
            Ok(emitter) => {
                spatial_world.emitters.insert(entity, emitter);
            }
            Err(err) => {
                error!("Cannot create spatial audio emitter for entity {entity:?}: {err}");
            }
        }
    }
}

fn update_listeners(
    mut spatial_world: ResMut<SpatialWorld>,
    q: Query<(Entity, &GlobalTransform), With<AudioListener>>,
) {
    for (entity, global_transform) in &q {
        let (_, quat, position) = global_transform.to_scale_rotation_translation();
        let Some(listener) = spatial_world.listeners.get_mut(&entity) else {
            continue;
        };
        listener.set_position(position, Tween::default()).unwrap();
        listener.set_orientation(quat, Tween::default()).unwrap();
    }
}
fn update_emitters(
    mut spatial_world: ResMut<SpatialWorld>,
    q: Query<(Entity, &GlobalTransform), (With<Audio>, With<SpatialEmitter>)>,
) {
    for (entity, global_transform) in &q {
        let Some(emitter) = spatial_world.emitters.get_mut(&entity) else {
            continue;
        };
        let position = global_transform.translation();
        match emitter.set_position(position, Tween::default()) {
            Ok(_) => {}
            Err(err) => {
                error!("Cannot set spatial audio position for entity {entity:?}: {err}");
            }
        }
    }
}
