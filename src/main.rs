mod camera;
mod car_fleet;
mod traffic_light;
mod ui_components;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Slow Motion
        // .insert_resource(Time::<Fixed>::from_hz(10.0))
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
            (traffic_light::update_event_emitter, traffic_light::update),
        )
        .add_systems(PreUpdate, traffic_light::on_scene_loaded)
        // Car Fleet
        .add_systems(Startup, car_fleet::setup)
        .add_systems(FixedUpdate, car_fleet::update)
        // UI
        .add_systems(Startup, ui_components::reset_simulation_button::setup)
        .add_systems(Update, ui_components::reset_simulation_button::update)
        // Simulation Reset
        .add_event::<ui_components::reset_simulation_button::ResetSimluation>()
        .add_systems(Update, camera::reset_simulation_listener)
        .add_systems(Update, car_fleet::reset_simulation_listener)
        .add_systems(Update, traffic_light::reset_simulation_listener)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { ..default() },
        transform: Transform::from_xyz(-4.0, -8.0, -4.0),
        ..default()
    });
}
