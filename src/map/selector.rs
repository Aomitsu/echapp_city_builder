use bevy::prelude::*;
use bevy_ecs_tilemap::{tiles::{TileStorage, TilePos}, map::{TilemapType, IsoCoordSystem}, helpers::square_grid::staggered::StaggeredPos};
use bevy_enum_filter::Enum;

use crate::{prelude::{input::mouse::MouseToWorldCoords, tilemap_layer_filters, world::CityWorld, TextureAssets}, map::TILEMAP_SIZE};

use super::{TILEMAP_TILE_SIZE, TILEMAP_TYPE, world::TilemapLayer};

#[derive(Component)]
pub struct SelectorEntity;

pub fn update_selector_pos(
    mut city_world_query: Query<&mut CityWorld>,
    mouse_world_coords: Res<MouseToWorldCoords>
) {
    let mut city_world = city_world_query.single_mut();

    let pos = TilePos::from_world_pos(&mouse_world_coords.0, &TILEMAP_SIZE, &TILEMAP_TILE_SIZE.into(), &TILEMAP_TYPE);

    if let Some(pos) = pos {
        let ivec = IVec2::new(pos.x as i32, pos.y as i32);
        if city_world.selector != Some(ivec) && ivec.x != 0 && ivec.y != 0 && ivec.x < city_world.map_size as i32 + 1 && ivec.y < city_world.map_size as i32 + 1 {
            city_world.selector = Some(ivec);
            debug!("cursor pos: x:{} y:{}", pos.x, pos.y);
        }
    }
}

pub fn create_selector(
    mut commands: Commands,
    assets: Res<TextureAssets>
) {
    commands.spawn(
        (
            SpriteBundle {
                texture: assets.texture_selector.clone(),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            SelectorEntity
        )
    );
}

pub fn update_selector(
    mut commands: Commands,
    mut selector_query: Query<(&mut Transform, &mut Visibility), With<SelectorEntity>>,
    city_world_query: Query<&CityWorld>,
) {
    let city_world = city_world_query.single();
    if let Some(selector) = city_world.selector {
        let world_pos = TilePos::new(selector.x as u32, selector.y as u32)
            .center_in_world(&TILEMAP_TILE_SIZE.into(), &TILEMAP_TYPE);
        let (mut transform, mut visible) = selector_query.single_mut();
        transform.translation = Vec3::new(world_pos.x, world_pos.y, TilemapLayer::Selector as i32 as f32);
        *visible = Visibility::Visible;
    }
    
}