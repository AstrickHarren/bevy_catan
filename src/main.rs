mod camera;
mod light;
mod load_asset;
mod map;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use camera::CameraPlugin;
use light::LightPlugin;
use load_asset::LoadAssetPlugin;
use map::MapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(LoadAssetPlugin)
        .add_plugins(LightPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(DefaultPickingPlugins)
        .run();
}
