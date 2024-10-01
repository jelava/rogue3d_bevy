mod components;
mod input;
mod map_gen;

use bevy::prelude::*;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};

use crate::server::{
    input::handle_player_input,
    map_gen::{systems::*, FloorGenerationParams},
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntropyPlugin::<WyRand>::default())
            .insert_resource(FloorGenerationParams::default())
            .add_systems(
                Startup,
                (
                    generate_abstract_floor,
                    (generate_blocks_from_rooms, spawn_creatures_in_rooms),
                )
                    .chain(),
            )
            .add_systems(Update, handle_player_input);
    }
}
