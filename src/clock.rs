use core::fmt;

use bevy::prelude::*;
use kira::{
    clock::ClockSpeed,
    tween::{Tween, Value},
};

use crate::AudioWorld;

/// Plugin integrating audio engine clocks as entities in Bevy.
pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                handle_added_clocks,
                handle_changed_clocks,
                handle_deleted_clocks,
            )
                .in_set(ClockSet::Update),
        );
    }
}

/// Set of all systems handling clocks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum ClockSet {
    /// Set of systems that update the audio engine from the ECS
    Update,
}

/// Settings for a clock. These settings can be modified and will be applied at every ECS tick.
///
/// Use the [`Clock`] component to query on the clock and get the handle back.
#[derive(Debug, Clone, Copy, Component)]
pub struct ClockSettings {
    /// Speed at which the clock ticks. See [`ClockSpeed`] for more details.
    pub speed: Value<ClockSpeed>,
}

/// Clock component
#[derive(Deref, Component)]
pub struct Clock(kira::clock::ClockHandle);

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(std::any::type_name::<Self>())
            .field(&self.0.id())
            .finish()
    }
}

/// Bundle for spawning clocks.
///
/// When query for this clock, use the [`Clock`] component, which allows control over the created
/// clock.
#[derive(Debug, Bundle)]
pub struct ClockBundle {
    /// The speed at which the clock is running
    pub settings: ClockSettings,
}

fn handle_added_clocks(
    mut commands: Commands,
    mut audio_world: ResMut<AudioWorld>,
    q: Query<(Entity, &ClockSettings), Added<ClockSettings>>,
) {
    for (entity, settings) in &q {
        let handle = audio_world.audio_manager.add_clock(settings.speed);
        match handle {
            Ok(handle) => {
                commands.entity(entity).insert(Clock(handle));
            }
            Err(err) => error!("Cannot add clock: {err}"),
        }
    }
}

fn handle_changed_clocks(q: Query<(&Clock, &ClockSettings), Changed<ClockSettings>>) {
    for (clock, clock_speed) in &q {
        // TODO: remove when updating Kira
        let _ = clock.0.set_speed(clock_speed.speed, Tween::default());
    }
}

fn handle_deleted_clocks(mut commands: Commands, mut q: RemovedComponents<ClockSettings>) {
    for entity in q.read() {
        commands.entity(entity).remove::<ClockBundle>();
    }
}
