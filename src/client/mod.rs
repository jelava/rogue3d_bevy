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
        app.add_systems(PreUpdate, (handle_creature_spawns, handle_block_spawns));
        app.add_systems(
            Update,
            (
                player_kb_input_mapper,
                handle_position_updates,
                (handle_camera_input, update_billboard_transforms).chain(),
            ),
        );
    }
}
