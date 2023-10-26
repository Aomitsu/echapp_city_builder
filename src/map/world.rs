use std::collections::HashMap;

use bevy::prelude::*;

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
        Self { blocks: Default::default(), selector: Default::default(), map_size: Default::default(), last_map_size: Default::default() }
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

#[derive(Debug, Default, Component, Reflect)]
pub enum TilemapLayer {
    Floor,
    Selector,
    #[default]
    None,
}