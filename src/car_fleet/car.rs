use bevy::{
    asset::Handle,
    ecs::{component::Component, system::Res},
    prelude::{Bundle, Mut},
    scene::{Scene, SceneBundle},
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
};

use crate::traffic_light::Light;

/*
The distance a car should leave before itself and the obstacle (other car/stoplight) in front of it
I'm pretty sure the car's position is the center of the car, so I'm setting this value to half_car_length + some offset
 */
const BREAK_DISTANCE: f32 = 6.0;

#[derive(Component)]
pub struct Car;

#[derive(Component)]
pub struct Acceleration(pub f32);

#[derive(Component)]
pub struct Velocity(pub f32);

#[derive(Component)]
pub struct ReactionTimer(pub Timer);

#[derive(Component)]
pub struct IsBreaking(pub bool);

const MAX_VELOCITY: f32 = 0.1;
const SPEED_UP_ACCELERATION: f32 = 0.003;
const SLOW_DOWN_ACCELERATION: f32 = -0.01;

#[derive(Bundle)]
pub struct CarBundle {
    scene: SceneBundle,
    car: Car,
    velocity: Velocity,
    acceleration: Acceleration,
    reaction_timer: ReactionTimer,
    is_breaking: IsBreaking,
}

pub fn get_car_bundle(
    car_asset: Handle<Scene>,
    transform: Transform,
    velocity: Option<f32>,
    acceleration: Option<f32>,
    reaction_time_in_seconds: Option<f32>,
) -> CarBundle {
    return CarBundle {
        scene: SceneBundle {
            scene: car_asset,
            transform,
            ..Default::default()
        },
        car: Car {},
        velocity: Velocity(velocity.unwrap_or(0.0f32)),
        acceleration: Acceleration(acceleration.unwrap_or(0.0f32)),
        reaction_timer: ReactionTimer(Timer::from_seconds(
            reaction_time_in_seconds.unwrap_or(0.4),
            TimerMode::Once,
        )),
        is_breaking: IsBreaking(false),
    };
}

pub fn apply_movement(
    car: (
        Mut<'_, Transform>,
        Mut<'_, Acceleration>,
        Mut<'_, Velocity>,
        Mut<'_, ReactionTimer>,
        Mut<'_, IsBreaking>,
    ),
    cars_z_positions: &Vec<f32>,
    time: &Res<Time>,
    current_traffic_light: Light,
    traffic_light_position: &Transform,
) {
    let (mut car_transform, mut acceleration, mut velocity, mut reaction_timer, mut is_breaking) =
        car;
    let should_break = should_break(
        car_transform.translation.z,
        velocity.0,
        &cars_z_positions,
        current_traffic_light,
        traffic_light_position,
    );
    if should_break {
        if !is_breaking.0 {
            is_breaking.0 = true;
            acceleration.0 = SLOW_DOWN_ACCELERATION
        }
    } else {
        if is_breaking.0 {
            is_breaking.0 = false;
            reaction_timer.0.reset();
        }
        reaction_timer.0.tick(time.delta());
        if reaction_timer.0.finished() {
            acceleration.0 = SPEED_UP_ACCELERATION;
        }
    }

    let new_velocity = velocity.0 + acceleration.0;
    if new_velocity > MAX_VELOCITY {
        velocity.0 = MAX_VELOCITY;
    } else if new_velocity < 0.0 {
        velocity.0 = 0.0;
    } else {
        velocity.0 = new_velocity;
    }
    car_transform.translation.z += velocity.0;
}

fn calculate_stopping_distance(current_velocity: f32) -> f32 {
    // I'm adding  0.1 to the stopping distance to avoid it being 0
    0.1 + (-current_velocity.powi(2)) / (2.0 * SLOW_DOWN_ACCELERATION)
}

fn get_car_infront_z_position(
    cars_z_positions: &Vec<f32>,
    current_car_z_position: f32,
) -> Option<f32> {
    let x = cars_z_positions
        .iter()
        .copied() // TODO: Not ideal, but the borrow-checker is giving me hell here
        .filter_map(|other_car_z_position| {
            if other_car_z_position > current_car_z_position {
                Some(other_car_z_position)
            } else {
                None
            }
        })
        .min_by(|a, b| a.partial_cmp(b).unwrap());
    return x;
}

fn should_break(
    position: f32,
    velocity: f32,
    other_cars_z_position: &Vec<f32>,
    current_traffic_light: Light,
    traffic_light_position: &Transform,
) -> bool {
    let minimum_distance_to_stop = calculate_stopping_distance(velocity);
    let car_infront_z_position = get_car_infront_z_position(&other_cars_z_position, position);
    match car_infront_z_position {
        Some(car_infront_z_position) => {
            if position + minimum_distance_to_stop + BREAK_DISTANCE >= car_infront_z_position {
                return true;
            }
        }
        None => {}
    }
    let before_traffic_light: bool =
        position + BREAK_DISTANCE <= traffic_light_position.translation.z;
    if before_traffic_light {
        match current_traffic_light {
            Light::RedLight => {
                if position + minimum_distance_to_stop + BREAK_DISTANCE
                    >= traffic_light_position.translation.z
                {
                    return true;
                }
                return false;
            }
            _ => {}
        }
    }
    return false;
}
