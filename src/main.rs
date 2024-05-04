mod camera;
mod light;
mod load_asset;
mod map;

use bevy::prelude::*;
use camera::CameraPlugin;
use light::LightPlugin;
use load_asset::LoadAssetPlugin;
use map::MapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LoadAssetPlugin)
        .add_plugins(LightPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .run();
}
