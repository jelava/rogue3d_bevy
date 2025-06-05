use bevy::prelude::*;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};
use components::{Collider, PlayerController};
use senses::sight::{update_sight, ClientSyncSight, Sight};
use server_sync::{ClientSync, LocalServerSyncPlugin, ServerSharedId};

use crate::{
    common::{
        grid::{GridPosition, GridShape},
        EntityKind,
    },
    server::input::handle_player_input,
};

mod components;
mod input;
pub mod server_sync;
// mod knowledge;
mod map_gen;
mod senses;

struct BaseServerPlugin;

impl Plugin for BaseServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntropyPlugin::<WyRand>::default())
            // .insert_resource(FloorGenerationParams::default())
            .add_systems(Startup, generate_test_level)
            .add_systems(PreUpdate, update_sight)
            .add_systems(Update, handle_player_input);
    }
}

// Temporary! just generate a single square room
fn generate_test_level(mut commands: Commands) {
    for x in 0..20 {
        for z in 0..20 {
            commands.spawn((
                ServerSharedId::new(),
                EntityKind::Block,
                GridPosition(IVec3::new(x, 0, z)),
                GridShape::SingleBlock,
                Collider,
            ));

            if x == 0 || x == 19 || z == 0 || z == 19 {
                commands.spawn((
                    ServerSharedId::new(),
                    EntityKind::Block,
                    GridPosition(IVec3::new(x, 1, z)),
                    GridShape::SingleBlock,
                    Collider,
                ));

                commands.spawn((
                    ServerSharedId::new(),
                    EntityKind::Block,
                    GridPosition(IVec3::new(x, 2, z)),
                    GridShape::SingleBlock,
                    Collider,
                ));
            }
        }
    }

    commands.spawn((
        ServerSharedId::new(),
        EntityKind::Player,
        ClientSync,
        PlayerController,
        GridPosition(IVec3::new(10, 1, 10)),
        GridShape::SingleBlock,
        Collider,
        Sight::new(4),
        ClientSyncSight,
    ));

    commands.spawn((
        ServerSharedId::new(),
        EntityKind::Npc,
        GridPosition(IVec3::new(5, 1, 7)),
        GridShape::SingleBlock,
        Collider,
        Sight::new(4),
    ));

    commands.spawn((
        ServerSharedId::new(),
        EntityKind::Npc,
        GridPosition(IVec3::new(12, 1, 14)),
        GridShape::SingleBlock,
        Collider,
        Sight::new(4),
    ));

    commands.spawn((
        ServerSharedId::new(),
        EntityKind::Npc,
        GridPosition(IVec3::new(17, 1, 4)),
        GridShape::SingleBlock,
        Collider,
        Sight::new(4),
        ClientSyncSight,
    ));

    commands.spawn((
        ServerSharedId::new(),
        EntityKind::Brazier,
        GridPosition(IVec3::new(10, 1, 15)),
        GridShape::SingleBlock,
        Collider,
    ));

    commands.spawn((
        ServerSharedId::new(),
        EntityKind::Brazier,
        GridPosition(IVec3::new(10, 1, 5)),
        GridShape::SingleBlock,
        Collider,
    ));

    commands.spawn((
        ServerSharedId::new(),
        EntityKind::Brazier,
        GridPosition(IVec3::new(15, 1, 10)),
        GridShape::SingleBlock,
        Collider,
    ));

    commands.spawn((
        ServerSharedId::new(),
        EntityKind::Brazier,
        GridPosition(IVec3::new(5, 1, 10)),
        GridShape::SingleBlock,
        Collider,
    ));
}

pub struct LocalServerPlugin;

impl Plugin for LocalServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BaseServerPlugin, LocalServerSyncPlugin));
    }
}
