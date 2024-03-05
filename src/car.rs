use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res},
    },
    math::Quat,
    scene::SceneBundle,
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
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

const MAX_VELOCITY: f32 = 0.1;
const BREAK_LINE_POSITION: f32 = 3.0;
const SPEED_UP_ACCELERATION: f32 = 0.003;
const SLOW_DOWN_ACCELERATION: f32 = -0.01;
const REACTION_TIME_IN_SECONDS: f32 = 0.5;
const SLOW_DOWN_BUFFER: f32 = 3.0; // The distance the car would stop towards a red light, in addition to the minimum distance to stop

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("car.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 10.0)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ..Default::default()
        },
        Car {},
        Velocity(0.0),
        Acceleration(0.0),
        ReactionTimer(Timer::from_seconds(
            REACTION_TIME_IN_SECONDS,
            TimerMode::Once,
        )),
    ));
}

pub fn apply_movement(
    mut car_q: Query<
        (
            &mut Transform,
            &mut Acceleration,
            &mut Velocity,
            &mut ReactionTimer,
        ),
        With<Car>,
    >,
    time: Res<Time>,
    traffic_light_q: Query<&TrafficLight>,
) {
    let (mut car_transform, mut acceleration, mut velocity, mut reaction_timer) =
        car_q.single_mut();
    let traffic_light = traffic_light_q.single();
    let before_traffic_light: bool = (car_transform.translation.z - BREAK_LINE_POSITION) > 0.0;
    if before_traffic_light {
        match traffic_light.current_light {
            Light::GreenLight => {
                acceleration.0 = SPEED_UP_ACCELERATION;
            }
            Light::RedLight => {
                let minimum_distance_to_stop = calculate_minimum_distance_to_stop(
                    velocity.0,
                    SLOW_DOWN_ACCELERATION,
                    REACTION_TIME_IN_SECONDS,
                );
                if car_transform.translation.z - BREAK_LINE_POSITION + SLOW_DOWN_BUFFER < minimum_distance_to_stop {
                    acceleration.0 = SPEED_UP_ACCELERATION;
                } else if car_transform.translation.z - minimum_distance_to_stop < SLOW_DOWN_BUFFER + BREAK_LINE_POSITION
                {
                    reaction_timer.0.tick(time.delta());
                    if reaction_timer.0.finished() {
                    reaction_timer.0.tick(time.delta());
                        reaction_timer.0.reset();
                        acceleration.0 = calculate_stopping_acceleration(
                            velocity.0,
                            car_transform.translation.z - BREAK_LINE_POSITION,
                        );
                    }
                } else {
                    acceleration.0 = SPEED_UP_ACCELERATION;
                }
            }
            _ => {}
        }
    } else {
        acceleration.0 = SPEED_UP_ACCELERATION;
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

// fn calculate_velocity(distance: f32, time: f32) -> f32 {
//     // FixedUpdate runs at 64Hz by default
//     distance / (time * 64.0)
// }

fn calculate_minimum_distance_to_stop(
    current_velocity: f32,
    acceleration: f32,
    reaction_time: f32,
) -> f32 {
    (current_velocity * reaction_time) + ((3.0 * current_velocity.powi(2)) / (2.0 * acceleration.abs()))
}

fn calculate_stopping_acceleration(current_velocity: f32, distance: f32) -> f32 {
    -current_velocity.powi(2) / (2.0 * distance)
}
