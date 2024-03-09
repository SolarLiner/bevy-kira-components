use crate::manager::AudioManager;
use bevy::diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic};
use bevy::prelude::*;

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

fn record_diagnostics(audio_manager: NonSend<AudioManager>, mut diagnostics: Diagnostics) {
    diagnostics.add_measurement(&NUM_COMMANDS, || {
        audio_manager.kira_manager.num_modulators() as _
    });
    diagnostics.add_measurement(&NUM_SOUNDS, || audio_manager.kira_manager.num_sounds() as _);
    diagnostics.add_measurement(&NUM_CLOCKS, || audio_manager.kira_manager.num_clocks() as _);
    diagnostics.add_measurement(&NUM_MODULATORS, || {
        audio_manager.kira_manager.num_modulators() as _
    });
}
