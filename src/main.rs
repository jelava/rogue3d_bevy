mod components;
mod input;
mod systems;

use bevy::prelude::*;
use input::{PlayerInputCommand, PlayerInputMap};

use {input::systems::*, systems::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<PlayerInputCommand>()
        .insert_resource(PlayerInputMap::default())
        .add_systems(Startup, (generate_blocks, spawn_creatures))
        .add_systems(Update, (player_kb_input_mapper, handle_player_input).chain())
        .add_systems(PostUpdate, update_billboard_transforms)
        .run();
}
