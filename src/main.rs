mod bridge;
mod client;
mod server;

use bevy::prelude::*;

use crate::{bridge::BridgePlugin, client::ClientPlugin, server::ServerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BridgePlugin)
        .add_plugins(ClientPlugin)
        .add_plugins(ServerPlugin)
        .run();
    //.add_event::<PlayerInputCommand>()

    // .insert_resource(PlayerInputMap::default())
    // .insert_resource(FloorGenerationParams::default())
    // .add_systems(
    //     Startup,
    //     (
    //         generate_abstract_floor,
    //         generate_blocks_from_rooms,
    //         spawn_creatures_in_rooms,
    //     )
    //         .chain(),
    // )
    // .add_systems(
    //     Update,
    //     (player_kb_input_mapper, handle_player_input).chain(),
    // )
    // .add_systems(
    //     PostUpdate,
    //     (
    //         /*camera_input_mapper,*/ handle_camera_input,
    //         update_billboard_transforms,
    //     )
    //         .chain(),
    // )
}
