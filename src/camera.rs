use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut cmds: Commands) {
    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 5., 0.))
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
