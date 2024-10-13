mod components;
mod input;
mod systems;

use bevy::{
    app::{Plugin, PreUpdate, Update},
    prelude::IntoSystemConfigs,
};

use crate::client::{
    input::{systems::*, PlayerInputMap},
    systems::*,
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PlayerInputMap::default());
        app.add_systems(
            Update,
            (
                player_kb_input_mapper,
                (handle_spawns, handle_position_updates).chain(),
                (handle_camera_input, update_billboard_transforms).chain(),
            ),
        );
    }
}
