use bevy::prelude::*;
use bevy_ecs_tilemap::{TilemapPlugin, prelude::*};

use crate::{GameState, loading::TextureAssets};

use self::world::{TilemapLayer, CityWorld};

pub mod world;
pub struct MapPlugin;

const MAX_MAP_SIZE: u32 = 256;
const TILEMAP_TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 64.0, y: 32.0 };
const TILEMAP_SIZE: TilemapSize = TilemapSize { x: MAX_MAP_SIZE, y: MAX_MAP_SIZE };

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .register_type::<world::CityWorld>()
            .register_type::<world::TilemapLayer>()
            .add_systems(OnEnter(GameState::Playing), startup)
            .add_systems(Update, refresh_map_size.run_if(in_state(GameState::Playing)));
    }
}
pub fn startup(mut commands: Commands, assets: Res<TextureAssets>) {
    /*let tilemap_selector_entity = commands.spawn(
        (TilemapBundle {
            size: TilemapSize {
                x: MAX_MAP_SIZE,
                y: MAX_MAP_SIZE,
            },
            texture: TilemapTexture::Single(assets.texture_selector.clone()),
            tile_size: TILEMAP_TILE_SIZE,
            ..Default::default()
        },
        TilemapLayer::Selector)
    ).id();**/

    commands.spawn(
        (TilemapBundle {
            grid_size: TILEMAP_TILE_SIZE.into(),
            size: TILEMAP_SIZE,
            texture: TilemapTexture::Single(assets.texture_floor.clone()),
            tile_size: TILEMAP_TILE_SIZE,
            storage: TileStorage::empty(TILEMAP_SIZE),
            map_type: TilemapType::Isometric(IsoCoordSystem::Diamond),
            ..Default::default()
        },
        TilemapLayer::Floor,
        CityWorld{
            map_size: 3,
            ..Default::default()
        })
    )
    //.add_child(tilemap_selector_entity)
    ;

}

pub fn refresh_map_size(
    mut commands: Commands,
    mut last_update_query: Query<(Entity, &mut CityWorld, &mut TileStorage)>
) {

    for (entity, mut world, mut storage) in last_update_query.iter_mut() {
        if world.map_size > MAX_MAP_SIZE || world.map_size < world.last_map_size {
            world.map_size = world.last_map_size;
        }
        if world.map_size != world.last_map_size {
            // update the map size in the storage
            // loop x and y + 1 because 0 is dedicated to decorations
            for x in 1..world.map_size + 1 {
                for y in 1..world.map_size + 1{
                    // get in storage if there is a tile, if not create a new one
                    let tile_pos = &TilePos { x, y };
                    let tile = storage.get(tile_pos);
                    if tile.is_none() {

                        let tile_entity = commands.spawn(TileBundle {
                            tilemap_id: TilemapId(entity),
                            position: *tile_pos,
                            texture_index: TileTextureIndex(0),
                            ..Default::default()
                        }).id();

                        storage.set(tile_pos, tile_entity)
                    }
                }
            }
            world.last_map_size = world.map_size;
        }
    }
}