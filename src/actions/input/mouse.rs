use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};

use crate::{
    camera::MainCamera,
    prelude::{AssignationTile, CityWorld},
};

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