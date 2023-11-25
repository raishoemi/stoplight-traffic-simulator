mod camera;
mod traffic_light;

use bevy::prelude::*;
use camera::{camera_control, setup_camera};
use traffic_light::{setup_traffic_light, change_traffic_light};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin))
        .add_systems(Startup, (setup, setup_camera, setup_traffic_light))
        .add_systems(Update, (camera_control, change_traffic_light))
        .run();
}

fn setup(mut commands: Commands) {
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
