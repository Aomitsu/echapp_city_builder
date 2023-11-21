use bevy::prelude::*;
use bevy_ecs_tilemap::{prelude::*, TilemapPlugin};
use bevy_enum_filter::prelude::AddEnumFilter;

use crate::GameState;

use self::world::TilemapLayer;

pub mod selector;
pub mod world;
pub struct MapPlugin;

const MAX_MAP_SIZE: u32 = 256;
const TILEMAP_TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 64.0, y: 32.0 };
const TILEMAP_SIZE: TilemapSize = TilemapSize {
    x: MAX_MAP_SIZE,
    y: MAX_MAP_SIZE,
};
const TILEMAP_TYPE: TilemapType = TilemapType::Isometric(IsoCoordSystem::Diamond);

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .register_type::<world::CityWorld>()
            .add_enum_filter::<TilemapLayer>()
            .add_systems(
                OnEnter(GameState::Playing),
                (world::spawn_tilemap, selector::create_selector),
            )
            .add_systems(
                Update,
                (
                    world::update_map_size,
                    world::update_assignation_layer_visibility,
                    selector::update_selector_pos,
                    selector::update_selector,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
