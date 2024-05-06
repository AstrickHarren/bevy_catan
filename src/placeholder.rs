use bevy::prelude::*;
use bevy_mod_picking::{
    highlight::{Highlight, HighlightKind},
    PickableBundle,
};

pub(crate) struct PlaceHolderPlugin;

impl Plugin for PlaceHolderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, init_placeholders);
    }
}

#[derive(Debug, Component)]
pub(crate) struct PlaceHolder;

const BLEND_ALPHA: HighlightKind<StandardMaterial> = HighlightKind::new_dynamic(|m| {
    let mut m = m.clone();
    m.base_color.set_a(0.8);
    m.alpha_mode = AlphaMode::Blend;
    m
});

const NO_ALPHA: HighlightKind<StandardMaterial> = HighlightKind::new_dynamic(|m| m.clone());

const PLACEHOLDER_HIGHLIGHT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(BLEND_ALPHA),
    pressed: Some(BLEND_ALPHA),
    selected: Some(NO_ALPHA),
};

fn init_placeholders(
    placeholders: Query<Entity, Added<PlaceHolder>>,
    children: Query<&Children>,
    mut mtls: Query<&mut Handle<StandardMaterial>>,
    mut assets: ResMut<Assets<StandardMaterial>>,
    mut cmds: Commands,
) {
    for e in &placeholders {
        let desc = children
            .iter_descendants(e)
            .filter(|e| mtls.get(*e).is_ok())
            .collect::<Vec<_>>();
        for d in desc {
            // transparent placeholder
            let mut mtl = mtls.get_mut(d).unwrap();
            let mut ass = assets.get(mtl.id()).unwrap().clone();
            ass.base_color.set_a(0.5);
            ass.alpha_mode = AlphaMode::Blend;
            *mtl = assets.add(ass);

            // add hover to placeholder
            cmds.entity(d)
                .insert(PickableBundle::default())
                .insert(PLACEHOLDER_HIGHLIGHT);
        }
    }
}
