use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::client::input::{PlayerInputCommand, PlayerInputMap};

pub fn player_kb_input_mapper(
    input_map: Res<PlayerInputMap>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut player_input_events: EventWriter<PlayerInputCommand>,
) {
    // if there are many events to process since the last time the system ran, don't try to process them all at once
    // just take the first one and ignore the rest
    if let Some(keycode) = kb_input.get_just_pressed().next() {
        if let Some(command) = input_map.get(keycode) {
            player_input_events.send(*command);
        }
    }
}

// TODO: create input mapper system for camera controls!
pub fn handle_camera_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut camera_transform = query.single_mut();

    if keyboard_input.pressed(KeyCode::KeyW) {
        let forward = 0.1 * camera_transform.forward().as_vec3();
        camera_transform.translation += forward;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        let back = 0.1 * camera_transform.back().as_vec3();
        camera_transform.translation += back;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        let right = 0.1 * camera_transform.right().as_vec3();
        camera_transform.translation += right;
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        let left = 0.1 * camera_transform.left().as_vec3();
        camera_transform.translation += left;
    } else if keyboard_input.pressed(KeyCode::KeyR) {
        let up = 0.1 * camera_transform.up().as_vec3();
        camera_transform.translation += up;
    } else if keyboard_input.pressed(KeyCode::KeyF) {
        let down = 0.1 * camera_transform.down().as_vec3();
        camera_transform.translation += down;
    }

    if mouse_button_input.pressed(MouseButton::Right) {
        if let Some(motion_event) = mouse_motion_events.read().next() {
            camera_transform.rotate_local_x(0.01 * -motion_event.delta.y);
            camera_transform.rotate_y(0.01 * -motion_event.delta.x);
        }
    }
}
