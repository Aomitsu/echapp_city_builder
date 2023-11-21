use bevy::prelude::*;

use crate::prelude::CityAssignation;

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct KeyboardToggle { // TODO: To refactor
    pub edit_mode: bool,
    pub assignement_to_set: CityAssignation,
}

pub fn game_keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut game_mode: ResMut<KeyboardToggle>,
) {
    // Edit mode switching
    if keys.just_pressed(KeyCode::T) {
        game_mode.edit_mode = !game_mode.edit_mode;
    }
}