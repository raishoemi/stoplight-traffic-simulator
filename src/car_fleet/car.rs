use bevy::{
    asset::Handle, ecs::{
        component::Component,
        query::With,
        system::{Query, Res},
    }, prelude::Bundle, scene::{Scene, SceneBundle}, time::{Time, Timer, TimerMode}, transform::components::Transform
};

use crate::traffic_light::{Light, TrafficLight};

#[derive(Component)]
pub struct Car;

#[derive(Component)]
pub struct Acceleration(f32);

#[derive(Component)]
pub struct Velocity(f32);

#[derive(Component)]
pub struct ReactionTimer(Timer);

#[derive(Component)]
pub struct IsBreaking(bool);

const MAX_VELOCITY: f32 = 0.1;
const BREAK_LINE_POSITION: f32 = 3.0;
const SPEED_UP_ACCELERATION: f32 = 0.003;
const SLOW_DOWN_ACCELERATION: f32 = -0.01;
const REACTION_TIME_IN_SECONDS: f32 = 0.5;
const SLOW_DOWN_BUFFER: f32 = 3.0; // The distance the car would stop towards a red light, in addition to the minimum distance to stop

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
            REACTION_TIME_IN_SECONDS,
            TimerMode::Once,
        )),
        is_breaking: IsBreaking(false)
    };
}

pub fn apply_movement(
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
    time: Res<Time>,
    traffic_light_q: Query<&TrafficLight>,
) {
    let cars_z_positions = car_q
        .transmute_lens::<&Transform>()
        .query()
        .iter()
        .map(|transform| transform.translation.z)
        .collect::<Vec<f32>>();
    let cars = car_q.iter_mut();
    for (mut car_transform, mut acceleration, mut velocity, mut reaction_timer, mut is_breaking) in cars {
        let break_position = should_break(car_transform.translation.z, velocity.0, cars_z_positions.clone(), traffic_light_q.single());
        if break_position > 0.0 {
            if !is_breaking.0 {
                is_breaking.0 = true;
                reaction_timer.0.reset();
            }   
            reaction_timer.0.tick(time.delta());
            if reaction_timer.0.finished() {
                reaction_timer.0.tick(time.delta());
                reaction_timer.0.reset();
                acceleration.0 = calculate_stopping_acceleration(
                    velocity.0,
                    break_position,
                );
            }
        } else {
            if is_breaking.0 {
                is_breaking.0 = false;
                reaction_timer.0.reset();
            }   
            reaction_timer.0.tick(time.delta());
            if reaction_timer.0.finished() {
                reaction_timer.0.tick(time.delta());
                reaction_timer.0.reset();
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
        car_transform.translation.z -= velocity.0;
    }
}

fn calculate_minimum_distance_to_stop(
    current_velocity: f32
) -> f32 {
    (current_velocity * REACTION_TIME_IN_SECONDS)
        + ((3.0 * current_velocity.powi(2)) / (2.0 * SLOW_DOWN_ACCELERATION.abs())) + SLOW_DOWN_BUFFER
}

fn calculate_stopping_acceleration(current_velocity: f32, distance: f32) -> f32 {
    -current_velocity.powi(2) / (2.0 * distance)
}

fn get_car_infront_z_position(
    cars_z_positions: &Vec<f32>,
    current_car_z_position: f32,
) -> Option<f32> {
    cars_z_positions
        .iter()
        .copied() // TODO: Not ideal, but the borrow-checker is giving me hell here
        .filter_map(|other_car_z_position| {
            if other_car_z_position > current_car_z_position {
                Some(other_car_z_position)
            } else {
                None
            }
        })
        .min_by(|a, b| a.partial_cmp(b).unwrap())
}

fn should_break(
    position: f32,
    velocity: f32,
    oter_cars_z_position: Vec<f32>,
    traffic_light: &TrafficLight,
) -> f32 {
    let minimum_distance_to_stop = calculate_minimum_distance_to_stop(
        velocity
    );
    let car_infront_z_position = get_car_infront_z_position(&oter_cars_z_position, position);
    match car_infront_z_position {
        Some(car_infront_z_position) => {
            if position - car_infront_z_position < minimum_distance_to_stop {
                return position - car_infront_z_position;
            }
        }
        None => {}
    }
    let before_traffic_light: bool = (position - BREAK_LINE_POSITION) > 0.0;
    if before_traffic_light {
        match traffic_light.current_light {
            Light::RedLight => {
                if position - BREAK_LINE_POSITION + SLOW_DOWN_BUFFER
                    < minimum_distance_to_stop
                {
                    return -1.0;
                } else if position - minimum_distance_to_stop
                    < BREAK_LINE_POSITION
                {
                    return position - BREAK_LINE_POSITION;
                }
            }
            _ => {}
        }
    }
    return -1.0;
}
