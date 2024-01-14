use bevy::{app::Plugin, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
                debug_ui
        );
    }
}

pub fn debug_ui(
    mut contexts: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
) {
    egui::SidePanel::left("DebugUI")
        .resizable(false)
        .min_width(200.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Debug UI");
            /*
               FPS / GRAPHIC ENGINE RELATED
            */
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                ui.label(format!("RAW FPS: {:?}", fps.value().unwrap_or(0.0).floor()));
                ui.label(format!(
                    "AVG FPS: {:?}",
                    fps.average().unwrap_or(0.0).floor()
                ));
                ui.label(format!(
                    "SMOOTH FPS: {:?}",
                    fps.smoothed().unwrap_or(0.0).floor()
                ));
            }
        });
}