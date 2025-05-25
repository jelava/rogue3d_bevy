mod common;

use common::*;

use bevy::prelude::*;
use rogue3d_bevy::{
    common::grid::GridPosition,
    server::server_sync::{ClientSync, LocalServerSyncPlugin, ServerSharedId},
};

#[test]
fn client_synced_movement_test() {
    App::new()
        .add_plugins((BaseTestPlugins, LocalServerSyncPlugin))
        .add_systems(
            Startup,
            (init_entities, check_init, move_entities, check_move).chain(),
        )
        .run();
}

fn init_entities(mut commands: Commands) {
    commands.spawn((ServerSharedId::new(), ClientSync, GridPosition(IVec3::ZERO)));
    commands.spawn((ServerSharedId::new(), GridPosition(IVec3::new(1, 1, 1))));
    commands.spawn((
        ServerSharedId::new(),
        ClientSync,
        GridPosition(IVec3::new(2, 2, 2)),
    ));
    commands.spawn((ServerSharedId::new(), GridPosition(IVec3::new(3, 3, 3))));
}

fn check_init() {
    todo!()
}

fn move_entities() {
    todo!()
}

fn check_move() {
    todo!()
}
