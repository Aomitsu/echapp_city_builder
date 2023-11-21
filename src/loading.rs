use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "icon.png")]
    pub texture_icon: Handle<Image>,
    #[asset(path = "textures/floor.png")]
    pub texture_floor: Handle<Image>,
    #[asset(path = "textures/assignment_overlay.png")]
    pub texture_assignment_overlay: Handle<Image>,
    #[asset(path = "textures/selector.png")]
    pub texture_selector: Handle<Image>,
    #[asset(path = "textures/starter-road.png")]
    pub texture_st_road: Handle<Image>,
}
