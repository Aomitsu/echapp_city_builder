use bevy::prelude::*;
use bevy_ecs_tilemap::{TilemapPlugin, prelude::*};

use crate::{GameState, loading::TextureAssets};

pub mod world;
pub struct MapPlugin;

const QUADRANT_SIDE_LENGTH: u32 = 80;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(OnEnter(GameState::Playing), startup);
    }
}

fn startup(mut commands: Commands, assets: Res<TextureAssets>) {
    // OpenTTD Default Map Size + 1
    let map_size = TilemapSize {
        x: 257,
        y: 257,
    };
    let quadrant_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH,
        y: QUADRANT_SIDE_LENGTH,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);
    
    fill_tilemap_rect(
        TileTextureIndex(2),
        TilePos { x: 3, y: 1 },
        TilemapSize { x: 1, y: 3 },
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );
    fill_tilemap_rect(
        TileTextureIndex(1),
        TilePos { x: 0, y: 0 },
        TilemapSize { x: 3, y: 1 },
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );
    fill_tilemap_rect(
        TileTextureIndex(0),
        TilePos { x: 0, y: 1 },
        TilemapSize { x: 3, y: 3 },
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 64.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    debug!("{:?}", tile_storage);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(assets.texture_floor.clone()),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}  