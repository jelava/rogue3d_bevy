mod components;
mod input;
mod map_gen;
mod systems;

use bevy::prelude::*;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};
use input::{PlayerInputCommand, PlayerInputMap};
use map_gen::FloorGenerationParams;

use {input::systems::*, map_gen::systems::*, systems::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_event::<PlayerInputCommand>()
        .insert_resource(PlayerInputMap::default())
        .insert_resource(FloorGenerationParams::default())
        .add_systems(
            Startup,
            (
                generate_abstract_floor,
                generate_blocks_from_rooms,
                spawn_creatures_in_rooms,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (player_kb_input_mapper, handle_player_input).chain(),
        )
        .add_systems(
            PostUpdate,
            (
                /*camera_input_mapper,*/ handle_camera_input,
                update_billboard_transforms,
            )
                .chain(),
        )
        .run();
}
