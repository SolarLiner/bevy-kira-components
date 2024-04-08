//! Support for spatial audio through `kira`'s spatial features.
use bevy::diagnostic::{Diagnostic, DiagnosticPath, RegisterDiagnostic};
use bevy::prelude::*;

use kira::spatial::emitter::{EmitterDistances, EmitterHandle, EmitterSettings};
use kira::spatial::listener::ListenerHandle;
use kira::spatial::scene::{SpatialSceneHandle, SpatialSceneSettings};
use kira::tween::{Easing, Tween};

use crate::{AudioPlaybackSet, AudioSourceSetup, AudioWorld, InternalAudioMarker};

#[doc(hidden)]
#[allow(missing_docs)]
pub mod prelude {
    pub use super::{AudioListener, SpatialEmitter, SpatialWorld};
}

/// Spatial audio plugin. This is an internal plugin, useful for some separation of concerns.
///
/// It is automatically added by the main [`AudioPlugin`].
pub(crate) struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialWorld>()
            .add_plugins(SpatialDiagnosticsPlugin)
            .add_systems(
                PreUpdate,
                (add_listeners, add_emitters)
                    .in_set(AudioPlaybackSet::Setup)
                    .before(AudioSourceSetup),
            )
            .add_systems(
                PostUpdate,
                (update_listeners, update_emitters).in_set(AudioPlaybackSet::Update),
            );
    }
}

/// Marker component setting this entity as an audio listener. It must have a [`GlobalTransform`]
/// attached for the spatial systems to pick it up.
#[derive(Component)]
pub struct AudioListener;

/// Internal handle to a Kira listener. Used to update the audio listener position.
#[derive(Component)]
pub(crate) struct SpatialListenerHandle(ListenerHandle);

/// Marker component setting this entity as a spatial emitter. It must have a [`GlobalTransform`]
/// attached for the spatial systems to pick it up.
///
/// Note that these settings are only used in the setup of the spatial emitter, and not kept in
/// sync afterwards.
#[derive(Component)]
pub struct SpatialEmitter {
    /// Function describing the attenuation in volume depending on the distance of this emitter
    /// to the listener.
    pub attenuation: Option<Easing>,
    /// Enables the panning effect that depends on the orientation of the listener.
    pub enable_spatialization: bool,
    /// Range of distances describing the distance at which the sound will be playing at full
    /// volume, and the maximum distance at which the sound will be able to be heard.
    pub distances: EmitterDistances,
}

impl Default for SpatialEmitter {
    fn default() -> Self {
        Self {
            attenuation: None,
            enable_spatialization: true,
            distances: EmitterDistances::default(),
        }
    }
}

/// Internal Kira handle emitter. Used to update the spatial emitter position.
#[derive(Component)]
pub(crate) struct SpatialEmitterHandle(pub(crate) EmitterHandle);

/// Global data related to spatial handling in the audio engine.
#[derive(Resource)]
pub struct SpatialWorld {
    pub(crate) spatial_handle: SpatialSceneHandle,
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
        Self { spatial_handle }
    }
}

fn add_listeners(
    mut commands: Commands,
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
        commands
            .entity(entity)
            .insert(SpatialListenerHandle(listener));
    }
}

fn add_emitters(
    mut commands: Commands,
    mut spatial_world: ResMut<SpatialWorld>,
    q: Query<(Entity, &GlobalTransform, &SpatialEmitter), Added<InternalAudioMarker>>,
) {
    for (entity, global_transform, spatial_emitter) in &q {
        let result = spatial_world.spatial_handle.add_emitter(
            global_transform.translation(),
            EmitterSettings::default()
                .attenuation_function(spatial_emitter.attenuation)
                .enable_spatialization(spatial_emitter.enable_spatialization)
                .distances(spatial_emitter.distances)
                .persist_until_sounds_finish(true),
        );
        debug!("Add emitter to {entity:?}");
        match result {
            Ok(emitter) => {
                commands
                    .entity(entity)
                    .insert(SpatialEmitterHandle(emitter));
            }
            Err(err) => {
                error!("Cannot create spatial audio emitter for entity {entity:?}: {err}");
            }
        }
    }
}

fn update_listeners(mut q: Query<(&mut SpatialListenerHandle, &GlobalTransform)>) {
    for (mut listener, global_transform) in &mut q {
        let (_, quat, position) = global_transform.to_scale_rotation_translation();
        listener.0.set_position(position, Tween::default()).unwrap();
        listener.0.set_orientation(quat, Tween::default()).unwrap();
    }
}

fn update_emitters(mut q: Query<(Entity, &mut SpatialEmitterHandle, &GlobalTransform)>) {
    for (entity, mut emitter, global_transform) in &mut q {
        let position = global_transform.translation();
        match emitter.0.set_position(position, Tween::default()) {
            Ok(_) => {}
            Err(err) => {
                error!("Cannot set spatial audio position for entity {entity:?}: {err}");
            }
        }
    }
}

/// Bevy diagnostic path recording the number of emitters present.
pub const SPATIAL_EMITTERS: DiagnosticPath = DiagnosticPath::const_new("kira::spatial::emitters");
/// Bevy diagnostic path recording the number of listeners present.
pub const SPATIAL_LISTENERS: DiagnosticPath = DiagnosticPath::const_new("kira::spatial::listeners");

struct SpatialDiagnosticsPlugin;

impl Plugin for SpatialDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.register_diagnostic(Diagnostic::new(SPATIAL_EMITTERS).with_suffix(" emitters"))
            .register_diagnostic(Diagnostic::new(SPATIAL_LISTENERS).with_suffix(" listeners"));
    }
}
