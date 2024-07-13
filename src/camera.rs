use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(7.0, 7.0, 7.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..default()
    });
}

pub fn update(
    mut query: Query<(&Camera, &mut Transform)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_scroll_evr: EventReader<MouseWheel>,
    mut mouse_motion_evr: EventReader<MouseMotion>,
) {
    let mut transform = query.single_mut().1;
    for event in mouse_motion_evr.read() {
        if mouse_buttons.pressed(MouseButton::Middle) {
            let pan_speed = 0.01;
            let pan = Vec3::new(-event.delta.x, event.delta.y, 0.0) * pan_speed;
            let rotation = transform.rotation.mul_vec3(pan);
            transform.translation += rotation;
        } else if mouse_buttons.pressed(MouseButton::Left) {
            let rotation_speed = 0.001;
            let yaw = -event.delta.x * rotation_speed;
            let pitch = -event.delta.y * rotation_speed;

            let yaw_rotation = Quat::from_rotation_y(yaw);
            let pitch_rotation = Quat::from_rotation_x(pitch);

            let rotation = yaw_rotation * pitch_rotation;
            transform.rotation = transform.rotation * rotation;
        }
    }
    for event in mouse_scroll_evr.read() {
        let zoom_speed = 0.5;
        let zoom = Vec3::new(0.0, 0.0, -event.y) * zoom_speed;
        let rotation = transform.rotation.mul_vec3(zoom);
        transform.translation += rotation;
    }
}
