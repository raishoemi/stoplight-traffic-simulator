use bevy::{prelude::*, gltf::*};

#[derive(Component)]
pub struct TrafficLight;

pub fn setup_traffic_light(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("traffic_light.gltf#Scene0"),
            visibility: Visibility::Visible,
            ..default()
        },
        TrafficLight
    ));
}

pub fn change_traffic_light(
    traffic_lights: Query<Entity, With<TrafficLight>>,
    children: Query<&Children>,
    name_query: Query<&Name>,
) {
    let traffic_light = traffic_lights.single();
    for child_entity in children.iter_descendants(traffic_light) {
        if let Ok(entity_name) = name_query.get(child_entity) {
            println!("{:?}", entity_name)
        }
    }
}
