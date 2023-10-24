use std::collections::HashMap;

use bevy::prelude::*;

pub struct CityMap {
    pub blocks: HashMap<IVec2, FloorType>,
    pub selector: Option<IVec2>,
    /// map_size x map_size, doesn't count decorative blocks under the map
    pub map_size: i32,

}

pub struct FloorType {
    pub assignation: CityAssignation,
}

#[derive(Debug, Default)]
pub enum CityAssignation {
    Residential,
    Commercial,
    Industrial,
    /// Can't be assigned ( Parks, Roads, Special infrastructures, )
    CantBeAssigned,
    #[default]
    Nothing,
}