use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{Listener, On},
};

use crate::{
    load_asset::{LoadState, PieceAsset},
    placeholder::PlaceHolder,
};

#[derive(Debug)]
pub(crate) struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LoadState::Loaded), allow_build_house);
    }
}

#[derive(Debug, Component)]
struct House;

fn allow_build_house(ass: Res<PieceAsset>, mut cmds: Commands) {
    let scene = SceneBundle {
        scene: ass.house.clone(),
        ..Default::default()
    };

    cmds.spawn(scene)
        .insert(PlaceHolder)
        .insert(On::<Pointer<Click>>::run(
            |this: Listener<Pointer<Click>>, mut cmds: Commands| {
                cmds.entity(this.listener())
                    .remove::<PlaceHolder>()
                    .insert(House);
            },
        ));
}
