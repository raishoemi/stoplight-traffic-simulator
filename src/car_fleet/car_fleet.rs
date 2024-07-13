use crate::traffic_light::TrafficLight;

use super::car::{self, get_car_bundle, Acceleration, Car, ReactionTimer, Velocity, IsBreaking};
use bevy::{
    asset::{AssetServer, Handle},
    math::Quat,
    prelude::{Commands, Query, Res, With},
    scene::Scene,
    time::Time,
    transform::components::Transform,
};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene: Handle<Scene> = asset_server.load("car.gltf#Scene0");
    for i in 1..=3 {
        commands.spawn(get_car_bundle(
            scene.clone(),
            Transform::from_xyz(0.0, 0.0, 10.0 * i as f32)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            None,
            None,
        ));
    }
}

pub fn update(
    car_q: Query<
        (
            &mut Transform,
            &mut Acceleration,
            &mut Velocity,
            &mut ReactionTimer,
            &mut IsBreaking,
        ),
        With<Car>,
    >,
    time: Res<Time>,
    traffic_light_q: Query<&TrafficLight>,
) {
    car::apply_movement(car_q, time, traffic_light_q);
}

