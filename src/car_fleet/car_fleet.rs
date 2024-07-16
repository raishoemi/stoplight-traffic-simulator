use crate::traffic_light::CurrentLight;
use bevy::prelude::Without;

use super::car::{self, get_car_bundle, Acceleration, Car, IsBreaking, ReactionTimer, Velocity};
use bevy::{
    asset::{AssetServer, Handle},
    prelude::{Commands, Query, Res, With},
    scene::Scene,
    time::Time,
    transform::components::Transform,
};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene: Handle<Scene> = asset_server.load("car.gltf#Scene0");
    for i in 1..=2 {
        commands.spawn(get_car_bundle(
            scene.clone(),
            Transform::from_xyz(0.0, 0.0, -10.0 * i as f32),
                //.with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            None,
            None,
        ));
    }
}

// TODO: The With/Without here is stupid, I should use ParamSets instead but the borrow-checker doesn't let me use both values at the same time
pub fn update(
    mut car_q: Query<
        (
            &mut Transform,
            &mut Acceleration,
            &mut Velocity,
            &mut ReactionTimer,
            &mut IsBreaking,
        ),
        With<Car>,
    >,
    traffic_light_q: Query<(&CurrentLight, &Transform), Without<Car>>,
    time: Res<Time>,
) {
    let (current_light, traffic_light_transform) = traffic_light_q.single();

    let cars_z_positions = car_q
        .transmute_lens::<&Transform>()
        .query()
        .iter()
        .map(|transform| transform.translation.z)
        .collect::<Vec<f32>>();
    for car in car_q.iter_mut() {
        car::apply_movement(
            car,
            &cars_z_positions,
            &time,
            current_light.0,
            traffic_light_transform,
        );
    }
}
