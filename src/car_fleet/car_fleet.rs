use std::time::Duration;

use crate::{
    traffic_light::CurrentLight,
    ui_components::{
        reaction_timer_controls::ReactionTimeChanged, reset_simulation_button::ResetSimluation,
    },
};
use bevy::prelude::{EventReader, Without};

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
    for i in 1..=6 {
        commands.spawn(get_car_bundle(
            scene.clone(),
            Transform::from_xyz(0.0, 0.0, -10.0 * i as f32),
            //.with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            None,
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

pub fn reset_simulation_listener(
    mut reset_simulation_event: EventReader<ResetSimluation>,
    mut query: Query<(
        &Car,
        &mut Transform,
        &mut Velocity,
        &mut Acceleration,
        &mut ReactionTimer,
        &mut IsBreaking,
    )>,
) {
    for _ in reset_simulation_event.read() {
        let mut i = 0;
        for mut car in query.iter_mut() {
            i += 1;
            *car.1 = Transform::from_xyz(0.0, 0.0, -10.0 * i as f32);
            *car.2 = Velocity(0.0);
            *car.3 = Acceleration(0.0);
            (car.4 .0).reset();
            *car.5 = IsBreaking(false);
        }
    }
}

pub fn reaction_time_changes_listener(
    mut reaction_time_changed_event: EventReader<ReactionTimeChanged>,
    mut query: Query<(&Car, &mut ReactionTimer)>,
) {
    for e in reaction_time_changed_event.read() {
        // At the  moment we change all the cars' reaction time
        for mut car in query.iter_mut() {
            let current_reaction_time = car.1 .0.duration().as_secs_f32();
            let reaction_timer = &mut car.1 .0;
            let new_reaction_time = current_reaction_time + e.delta;
            if new_reaction_time > 0.0 {
                reaction_timer.set_duration(Duration::from_secs_f32(new_reaction_time));
            }
        }
    }
}
