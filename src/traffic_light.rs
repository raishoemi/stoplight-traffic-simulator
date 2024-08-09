use core::fmt;

use bevy::prelude::*;

use crate::ui_components::reset_simulation_button::ResetSimluation;

#[derive(Debug, Clone, Copy)]
pub enum Light {
    RedLight,
    GreenLight,
    YellowLight,
}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Light::RedLight => write!(f, "RedLight"),
            Light::GreenLight => write!(f, "GreenLight"),
            Light::YellowLight => write!(f, "YellowLight"),
        }
    }
}

#[derive(Component)]
pub struct TrafficLight;

#[derive(Component)]
pub struct CurrentLight(pub Light);

#[derive(Component)]
pub struct HandleId(AssetId<Scene>);

#[derive(Component)]
pub struct LightChangeTimer {
    go: Timer,
    stop: Timer,
    yellow: Timer,
}

#[derive(Event)]
pub struct LightChange {
    pub light: Light,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_writer: EventWriter<LightChange>,
) {
    let asset_path = "traffic_light.gltf#Scene0";
    let scene_handle: Handle<Scene> = asset_server.load(asset_path);
    commands.spawn((
        SceneBundle {
            scene: scene_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ..Default::default()
        },
        TrafficLight {},
        CurrentLight { 0: Light::RedLight },
        HandleId {
            0: scene_handle.id(),
        },
        LightChangeTimer {
            go: Timer::from_seconds(10.0, TimerMode::Once),
            stop: Timer::from_seconds(10.0, TimerMode::Once),
            yellow: Timer::from_seconds(1.0, TimerMode::Once),
        },
    ));
    // TODO: This event doesn't do anything, we need to wait for the asset to load
    event_writer.send(LightChange {
        light: Light::RedLight,
    });
}

pub fn update_event_emitter(
    mut traffic_light_q: Query<(&CurrentLight, &mut LightChangeTimer), With<TrafficLight>>,
    time: Res<Time>,
    mut event_writer: EventWriter<LightChange>,
) {
    let (current_light, mut light_change_timers) = traffic_light_q.single_mut();

    match current_light.0 {
        Light::RedLight => {
            light_change_timers.stop.tick(time.delta());
            if light_change_timers.stop.finished() {
                event_writer.send(LightChange {
                    light: Light::GreenLight,
                });
            }
        }
        Light::YellowLight => {
            light_change_timers.yellow.tick(time.delta());
            if light_change_timers.yellow.finished() {
                event_writer.send(LightChange {
                    light: Light::RedLight,
                });
            }
        }
        Light::GreenLight => {
            light_change_timers.go.tick(time.delta());
            if light_change_timers.go.finished() {
                event_writer.send(LightChange {
                    light: Light::YellowLight,
                });
            }
        }
    }
}

pub fn update(
    mut light_change_events: EventReader<LightChange>,
    traffic_lights_entity_query: Query<Entity, With<TrafficLight>>,
    mut traffic_light_q: Query<(&mut CurrentLight, &mut LightChangeTimer), With<TrafficLight>>,
    children: Query<&Children>,
    mut child_query: Query<(&Name, &mut Visibility)>,
) {
    let traffic_light_entity = traffic_lights_entity_query.single();
    let (mut current_light, mut light_change_timer) = traffic_light_q.single_mut();
    for new_light in light_change_events.read() {
        current_light.0 = new_light.light;
        light_change_timer.go.reset();
        light_change_timer.yellow.reset();
        light_change_timer.stop.reset();
        for child_entity in children.iter_descendants(traffic_light_entity) {
            if let Ok((entity_name, mut visible)) = child_query.get_mut(child_entity) {
                let entity_name = entity_name.to_string();
                if entity_name == Light::RedLight.to_string()
                    || entity_name == Light::GreenLight.to_string()
                    || entity_name == Light::YellowLight.to_string()
                {
                    if entity_name == new_light.light.to_string() {
                        *visible = Visibility::Visible;
                    } else {
                        *visible = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

pub fn on_scene_loaded(
    mut ev_asset: EventReader<AssetEvent<Scene>>,
    traffic_light_q: Query<(&HandleId, &CurrentLight), With<TrafficLight>>,
    mut event_writer: EventWriter<LightChange>,
) {
    for ev in ev_asset.read() {
        match ev {
            AssetEvent::Added { id, .. } => {
                let (traffic_light_handle_id, current_light) = traffic_light_q.single();
                if traffic_light_handle_id.0 == *id {
                    event_writer.send(LightChange {
                        light: current_light.0.clone(),
                    });
                }
            }
            _ => {}
        }
    }
}

pub fn reset_simulation_listener(
    mut reset_simulation_event: EventReader<ResetSimluation>,
    mut event_writer: EventWriter<LightChange>,
) {
    for _ in reset_simulation_event.read() {
        event_writer.send(LightChange {
            light: Light::RedLight,
        });
    }
}
