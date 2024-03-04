mod camera;
mod car;
mod traffic_light;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<traffic_light::LightChange>()
        // .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin))
        .add_systems(Startup, setup)

        // Camera
        .add_systems(Startup, camera::setup)
        .add_systems(Update, camera::update)

        // Tarffic Light
        .add_systems(Startup, traffic_light::setup)
        .add_systems(
            Update,
            (
                traffic_light::update_event_emitter,
                traffic_light::update,
            ),
        )

        // // Car
        .add_systems(Startup, car::setup)
        .add_systems(FixedUpdate, car::apply_movement)
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
