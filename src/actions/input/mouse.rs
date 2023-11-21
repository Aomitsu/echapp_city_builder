use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};

use crate::{camera::MainCamera, prelude::{CityWorld, AssignationTile}};

use super::keyboard::KeyboardToggle;

#[derive(Resource, Default)]
pub struct MouseToWorldCoords(pub Vec2);

pub fn cursor_to_world_system(
    mut coords: ResMut<MouseToWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    query_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = query_camera.single();
    let window = query_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if world_position != coords.0 {
            coords.0 = world_position;
        }
    }
}

pub fn edit_mode_click_system( // Todo: Change place, it's the wrong "plugin"
    buttons: Res<Input<MouseButton>>,
    game_mode: Res<KeyboardToggle>,
    city_world_query: Query<&CityWorld>,
    mut assignation_layer_query: Query<(
        &TilePos, 
        &mut TileTextureIndex,
        With<AssignationTile>
    )>
) {
    if game_mode.edit_mode {
        if buttons.just_pressed(MouseButton::Left) {
            // Loop inside query
            let city_world = city_world_query.single();
            for (tile_pos, mut tile_texture_index, _) in assignation_layer_query.iter_mut() {
                if tile_pos.x == city_world.selector.unwrap().x as u32 && tile_pos.y == city_world.selector.unwrap().y as u32 { // TODO: Don't use unwra
                    *tile_texture_index = TileTextureIndex(game_mode.assignement_to_set as u32);
                    return;
                }
            }
        }
    }
}