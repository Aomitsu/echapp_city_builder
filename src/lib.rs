use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod actions;
mod camera;
mod loading;
mod map;
mod menu;
#[cfg(debug_assertions)]
mod debug;

use actions::ActionPlugin;
use camera::CameraPlugin;
use loading::LoadingPlugin;
use map::MapPlugin;
use menu::MenuPlugin;
#[cfg(debug_assertions)]
use debug::DebugPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            MapPlugin,
            CameraPlugin,
            ActionPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::default(),
                WorldInspectorPlugin::new(),
                DebugPlugin,
            ));
        }
    }
}

pub mod prelude {
    pub use crate::actions::*;
    pub use crate::camera::*;
    pub use crate::loading::*;
    pub use crate::map::world::*;
    pub use crate::map::*;
    pub use crate::menu::*;
}
