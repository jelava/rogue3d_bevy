mod client;
mod common;
mod server;

use bevy::prelude::{App, DefaultPlugins};

use crate::{client::ClientPlugin, common::BridgePlugin, server::ServerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BridgePlugin)
        .add_plugins(ClientPlugin)
        .add_plugins(ServerPlugin)
        .run();
}
