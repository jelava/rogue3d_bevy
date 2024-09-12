use bevy::prelude::*;

use crate::{
    components::{Collider, GridPosition, PlayerController},
    input::{PlayerInputCommand, PlayerInputMap},
};

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

pub fn handle_player_input(
    mut player_input_commands: EventReader<PlayerInputCommand>,
    mut player_transform_query: Query<(&mut GridPosition, &mut Transform), (With<PlayerController>, With<Collider>)>,
    occupied_blocks_query: Query<&GridPosition, (With<Collider>, Without<PlayerController>)>,
) {
    use PlayerInputCommand::*;

    let (mut player_grid_pos, mut player_transform) = player_transform_query.single_mut();

    if let Some(command) = player_input_commands.read().next() {
        println!("reading input cmd...");

        match *command {
            Walk(direction) => {
                let updated_translation = player_transform.translation + direction;

                if is_block_unoccupied(updated_translation) {
                    player_grid_pos = to_grid_indices(updated_translation);
                    player_transform.translation = updated_translation;
                }
            }
        }
    }
}

fn is_block_unoccupied(pos: Vec3) -> bool {
    true
} 

// pub fn handle_camera_input(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut Transform, With<Camera>>,
// ) {
//     let mut camera_transform = query.single_mut();

//     if keyboard_input.pressed(KeyCode::KeyW) {
//         let forward = 0.1 * camera_transform.forward().as_vec3();
//         camera_transform.translation += forward;
//     } else if keyboard_input.pressed(KeyCode::KeyS) {
//         let back = 0.1 * camera_transform.back().as_vec3();
//         camera_transform.translation += back;
//     } else if keyboard_input.pressed(KeyCode::KeyD) {
//         let right = 0.1 * camera_transform.right().as_vec3();
//         camera_transform.translation += right;
//     } else if keyboard_input.pressed(KeyCode::KeyA) {
//         let left = 0.1 * camera_transform.left().as_vec3();
//         camera_transform.translation += left;
//     }
// }
