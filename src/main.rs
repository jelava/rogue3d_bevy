mod client;
mod common;
mod server;

use bevy::prelude::App;

use crate::{client::LocalClientPlugin, common::LocalBridgePlugin, server::LocalServerPlugin};

fn main() {
    App::new()
        .add_plugins((LocalClientPlugin, LocalBridgePlugin, LocalServerPlugin))
        .run();
}
