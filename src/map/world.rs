use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapTexture},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};
use bevy_enum_filter::EnumFilter;

use crate::prelude::{input::keyboard::KeyboardToggle, TextureAssets};

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

#[derive(Debug, Default, Component, Reflect, Clone, Copy)]
pub enum CityAssignation {
    Residential = 0, // Orange
    Commercial = 2,  // Blue
    Industrial = 1,  // Yellow
    /// Can't be assigned ( Parks, Roads, Special infrastructures, )
    CantBeAssigned = 4,
    #[default]
    None = 3,
}

#[derive(Component)]
pub struct AssignationLayer;

#[derive(Component)]
pub struct AssignationTile;

#[derive(Default, Component, EnumFilter)]
pub enum TilemapLayer {
    Floor = 1,
    Assignation = 2,
    Selector = 3,
    Assets = 4,
    #[default]
    None = 5,
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
            transform: Transform::from_xyz(0.0, 0.0, TilemapLayer::Floor as i32 as f32),
            ..Default::default()
        },
        TilemapLayer::Floor,
        CityWorld {
            map_size: 3,
            ..Default::default()
        },
    ));
    commands.spawn((
        TilemapBundle {
            grid_size: TILEMAP_TILE_SIZE.into(),
            size: TILEMAP_SIZE,
            texture: TilemapTexture::Single(assets.texture_assignment_overlay.clone()),
            tile_size: TILEMAP_TILE_SIZE,
            storage: TileStorage::empty(TILEMAP_SIZE),
            map_type: TILEMAP_TYPE,
            transform: Transform::from_xyz(0.0, 0.0, TilemapLayer::Assignation as i32 as f32),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        TilemapLayer::Assignation,
        AssignationLayer, // TODO: need to use TilemapLayer::Assignation in the future
    ));
}

pub fn update_map_size(
    mut commands: Commands,
    mut storage_query: Query<(Entity, &mut CityWorld, &mut TileStorage), With<CityWorld>>,
    mut assignation_layer_query: Query<(
        Entity,
        &mut TileStorage,
        With<AssignationLayer>,
        Without<CityWorld>,
    )>,
) {
    let (entity, mut world, mut storage) = storage_query.single_mut();
    let (assignation_entity, mut assignation_storage, (), ()) =
        assignation_layer_query.single_mut();
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
                    let tile_assignment_entity = commands
                        .spawn((
                            TileBundle {
                                tilemap_id: TilemapId(assignation_entity),
                                position: *tile_pos,
                                texture_index: TileTextureIndex(CityAssignation::None as u32),
                                ..Default::default()
                            },
                            AssignationTile,
                        ))
                        .id();

                    storage.set(tile_pos, tile_entity);
                    assignation_storage.set(tile_pos, tile_assignment_entity);
                }
            }
        }
        world.last_map_size = world.map_size;
    }
}

pub fn update_assignation_layer_visibility(
    mut visibility_query: Query<&mut Visibility, With<AssignationLayer>>,
    status: Res<KeyboardToggle>,
) {
    let mut layer = visibility_query.single_mut();

    if status.edit_mode {
        *layer = Visibility::Visible;
    } else {
        *layer = Visibility::Hidden;
    }
}
