mod camera;

use bevy::prelude::*;
use camera::{camera_control, setup_camera};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin))
        .add_systems(Startup, (setup, setup_camera))
        .add_systems(Update, camera_control)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("car.gltf#Scene0"),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
