use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::{GameState, prelude::input::mouse};

use self::input::{mouse::{cursor_to_world_system, MouseToWorldCoords}, keyboard::{KeyboardToggle, game_keyboard_input}};

pub mod input;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseToWorldCoords>()
            .init_resource::<KeyboardToggle>()
            .add_plugins(ResourceInspectorPlugin::<KeyboardToggle>::default())
            .add_systems(Update, cursor_to_world_system)
            .add_systems(
                Update,
                (
                    game_keyboard_input,
                    mouse::edit_mode_click_system
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
