use bevy::diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic};
use bevy::prelude::*;
use crate::AudioWorld;

pub struct KiraStatisticsDiagnosticPlugin;

impl Plugin for KiraStatisticsDiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app.register_diagnostic(Diagnostic::new(NUM_COMMANDS).with_suffix(" commands"))
            .register_diagnostic(Diagnostic::new(NUM_SOUNDS).with_suffix(" sounds"))
            .register_diagnostic(Diagnostic::new(NUM_CLOCKS).with_suffix(" clocks"))
            .register_diagnostic(Diagnostic::new(NUM_MODULATORS).with_suffix(" modulators"))
            .add_systems(Last, record_diagnostics);
    }
}

const NUM_COMMANDS: DiagnosticPath = DiagnosticPath::const_new("kira::manager::num_commands");
const NUM_SOUNDS: DiagnosticPath = DiagnosticPath::const_new("kira::manager::num_sounds");
const NUM_CLOCKS: DiagnosticPath = DiagnosticPath::const_new("kira::manager::num_clocks");
const NUM_MODULATORS: DiagnosticPath = DiagnosticPath::const_new("kira::manager::num_modulators");

fn record_diagnostics(audio_world: Res<AudioWorld>, mut diagnostics: Diagnostics) {
    diagnostics.add_measurement(&NUM_COMMANDS, || {
        audio_world.audio_manager.num_modulators() as _
    });
    diagnostics.add_measurement(&NUM_SOUNDS, || audio_world.audio_manager.num_sounds() as _);
    diagnostics.add_measurement(&NUM_CLOCKS, || audio_world.audio_manager.num_clocks() as _);
    diagnostics.add_measurement(&NUM_MODULATORS, || {
        audio_world.audio_manager.num_modulators() as _
    });
}
