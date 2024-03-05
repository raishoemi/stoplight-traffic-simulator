use core::fmt;

use bevy::prelude::*;

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

// impl Copy for Light { }
// impl Clone for Light {
//     fn clone(&self) -> Light {
//         *self
//     }
// }

#[derive(Component)]
pub struct TrafficLight {
    pub current_light: Light,
    pub handle_id: AssetId<Scene>,
}

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
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        TrafficLight {
            current_light: Light::RedLight,
            handle_id: scene_handle.id(),
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
    mut traffic_lights_query: Query<(&mut TrafficLight, &mut LightChangeTimer)>,
    time: Res<Time>,
    mut event_writer: EventWriter<LightChange>,
) {
    let (mut traffic_light, mut light_change_timers) = traffic_lights_query.single_mut();

    match traffic_light.current_light {
        Light::RedLight => {
            light_change_timers.stop.tick(time.delta());
            if light_change_timers.stop.finished() {
                light_change_timers.stop.reset();
                traffic_light.current_light = Light::GreenLight;
                event_writer.send(LightChange {
                    light: Light::GreenLight,
                });
            }
        }
        Light::YellowLight => {
            light_change_timers.yellow.tick(time.delta());
            if light_change_timers.yellow.finished() {
                light_change_timers.yellow.reset();
                traffic_light.current_light = Light::RedLight;
                event_writer.send(LightChange {
                    light: Light::RedLight,
                });
            }
        }
        Light::GreenLight => {
            light_change_timers.go.tick(time.delta());
            if light_change_timers.go.finished() {
                light_change_timers.go.reset();
                traffic_light.current_light = Light::YellowLight;
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
    children: Query<&Children>,
    mut child_query: Query<(&Name, &mut Visibility)>,
) {
    let traffic_light_entity = traffic_lights_entity_query.single();
    for new_light in light_change_events.read() {
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
    traffic_light_query: Query<&TrafficLight>,
    mut event_writer: EventWriter<LightChange>,
) {
    for ev in ev_asset.read() {
        match ev {
            AssetEvent::Added { id, .. } => {
                let traffic_light = traffic_light_query.single();
                if traffic_light.handle_id == *id {
                    event_writer.send(LightChange {
                        light: traffic_light.current_light.clone(),
                    });
                }
            }
            _ => {}
        }
    }
}
