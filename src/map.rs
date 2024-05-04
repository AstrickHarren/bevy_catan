use bevy::prelude::*;

use crate::load_asset::{LoadState, TileAsset};

pub(crate) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LoadState::Loaded), spwan_map);
    }
}

fn spwan_map(mut cmds: Commands, ass: Res<TileAsset>) {
    cmds.spawn(SceneBundle {
        scene: ass.pasture.clone(),
        ..Default::default()
    });
}
