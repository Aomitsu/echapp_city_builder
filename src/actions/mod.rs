use bevy::prelude::*;

use self::input::mouse::{cursor_to_world_system, MouseToWorldCoords};

pub mod input;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseToWorldCoords>()
            .add_systems(Update, cursor_to_world_system);
    }
}
