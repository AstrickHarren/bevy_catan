use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Copy, Hash)]
pub(crate) enum LoadState {
    #[default]
    Loading,
    Loaded,
}

#[derive(AssetCollection, Resource)]
pub(crate) struct TileAsset {
    #[asset(path = "tiles/building-sheep.glb#Scene0")]
    pub pasture: Handle<Scene>,
}

#[derive(AssetCollection, Resource)]
pub(crate) struct PieceAsset {
    #[asset(path = "pieces/unit-house.glb#Scene0")]
    pub house: Handle<Scene>,
}

pub(crate) struct LoadAssetPlugin;

impl Plugin for LoadAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LoadState>().add_loading_state(
            LoadingState::new(LoadState::Loading)
                .continue_to_state(LoadState::Loaded)
                .load_collection::<PieceAsset>()
                .load_collection::<TileAsset>(),
        );
    }
}
