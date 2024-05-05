//! Register diagnostics regarding statistics of `Kira`'s audio engine usage.
#![cfg(feature = "diagnostics")]
use crate::AudioWorld;
use bevy::diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic};

use bevy::prelude::*;

/// Register diagnostics regarding statistics of `Kira`'s audio engine usage.
pub struct KiraStatisticsDiagnosticPlugin;

impl Plugin for KiraStatisticsDiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app.register_diagnostic(Diagnostic::new(COMMAND_COUNT).with_suffix(" commands"))
            .register_diagnostic(Diagnostic::new(SOUND_COUNT).with_suffix(" sounds"))
            .register_diagnostic(Diagnostic::new(CLOCK_COUNT).with_suffix(" clocks"))
            .register_diagnostic(Diagnostic::new(MODULATOR_COUNT).with_suffix(" modulators"))
            .register_diagnostic(
                Diagnostic::new(SPATIAL_SCENE_COUNT).with_suffix(" spatial scenes"),
            )
            .add_systems(Last, record_diagnostics);
    }
}

const COMMAND_COUNT: DiagnosticPath = DiagnosticPath::const_new("kira::manager::command_count");
const SOUND_COUNT: DiagnosticPath = DiagnosticPath::const_new("kira::manager::sound_count");
const CLOCK_COUNT: DiagnosticPath = DiagnosticPath::const_new("kira::manager::clock_count");
const MODULATOR_COUNT: DiagnosticPath = DiagnosticPath::const_new("kira::manager::modulator_count");
const SPATIAL_SCENE_COUNT: DiagnosticPath =
    DiagnosticPath::const_new("kira::manager::spatial_scene_count");

fn record_diagnostics(audio_world: Res<AudioWorld>, mut diagnostics: Diagnostics) {
    diagnostics.add_measurement(&COMMAND_COUNT, || {
        audio_world.audio_manager.num_modulators() as _
    });
    diagnostics.add_measurement(&SOUND_COUNT, || audio_world.audio_manager.num_sounds() as _);
    diagnostics.add_measurement(&CLOCK_COUNT, || audio_world.audio_manager.num_clocks() as _);
    diagnostics.add_measurement(&MODULATOR_COUNT, || {
        audio_world.audio_manager.num_modulators() as _
    });
    diagnostics.add_measurement(&SPATIAL_SCENE_COUNT, || {
        audio_world.audio_manager.num_spatial_scenes() as _
    });
}
