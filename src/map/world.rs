use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapTexture},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};
use bevy_enum_filter::EnumFilter;

use crate::prelude::TextureAssets;

use super::{MAX_MAP_SIZE, TILEMAP_SIZE, TILEMAP_TILE_SIZE, TILEMAP_TYPE};

#[derive(Component, Reflect)]
pub struct CityWorld {
    pub blocks: HashMap<IVec2, FloorType>,
    pub selector: Option<IVec2>,
    /// map_size x map_size, doesn't count decorative blocks under the map
    /// Maxmap size is hardcoded to 257*257 ( 256*256 without decorations)
    pub map_size: u32,
    pub last_map_size: u32,
}

impl Default for CityWorld {
    fn default() -> Self {
        Self {
            blocks: Default::default(),
            selector: Default::default(),
            map_size: Default::default(),
            last_map_size: Default::default(),
        }
    }
}

#[derive(Component, Reflect)]
pub struct FloorType {
    pub assignation: CityAssignation,
}

#[derive(Debug, Default, Component, Reflect)]
pub enum CityAssignation {
    Residential,
    Commercial,
    Industrial,
    /// Can't be assigned ( Parks, Roads, Special infrastructures, )
    CantBeAssigned,
    #[default]
    None,
}

#[derive(Debug, Default, Component, EnumFilter, Reflect)]
pub enum TilemapLayer {
    Floor = 1,
    Selector = 2,
    Assets = 3,
    #[default]
    None = 4,
}

pub fn spawn_tilemap(mut commands: Commands, assets: Res<TextureAssets>) {
    commands.spawn((
        TilemapBundle {
            grid_size: TILEMAP_TILE_SIZE.into(),
            size: TILEMAP_SIZE,
            texture: TilemapTexture::Single(assets.texture_floor.clone()),
            tile_size: TILEMAP_TILE_SIZE,
            storage: TileStorage::empty(TILEMAP_SIZE),
            map_type: TILEMAP_TYPE,
            ..Default::default()
        },
        TilemapLayer::Floor,
        CityWorld {
            map_size: 3,
            ..Default::default()
        },
    ));
}

pub fn update_map_size(
    mut commands: Commands,
    mut last_update_query: Query<(Entity, &mut CityWorld, &mut TileStorage)>,
) {
    let (entity, mut world, mut storage) = last_update_query.single_mut();
    if world.map_size > MAX_MAP_SIZE || world.map_size < world.last_map_size {
        world.map_size = world.last_map_size;
    }
    if world.map_size != world.last_map_size {
        // update the map size in the storage
        // loop x and y + 1 because 0 is dedicated to decorations
        for x in 1..world.map_size + 1 {
            for y in 1..world.map_size + 1 {
                // get in storage if there is a tile, if not create a new one
                let tile_pos = &TilePos { x, y };
                let tile = storage.get(tile_pos);
                if tile.is_none() {
                    let tile_entity = commands
                        .spawn(TileBundle {
                            tilemap_id: TilemapId(entity),
                            position: *tile_pos,
                            texture_index: TileTextureIndex(0),
                            ..Default::default()
                        })
                        .id();

                    storage.set(tile_pos, tile_entity)
                }
            }
        }
        world.last_map_size = world.map_size;
    }
}
