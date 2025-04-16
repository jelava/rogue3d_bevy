mod client;
mod common;
mod server;

use bevy::prelude::App;

use crate::{client::ClientPlugin, common::BridgePlugin, server::ServerPlugin};

fn main() {
    App::new()
        .add_plugins(ClientPlugin)
        .add_plugins(BridgePlugin)
        .add_plugins(ServerPlugin)
        .run();
}
