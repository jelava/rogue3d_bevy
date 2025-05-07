use bevy::prelude::*;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};
use components::{Collider, PlayerController};

use crate::{
    common::{
        grid::{GridPosition, GridShape, SparseGridIndexPlugin}, index::unique::{UniqueComponentIndexPlugin, UniqueSparseComponentIndex}, ClientUpdate, EntityKind, SharedId
    },
    server::input::handle_player_input,
};

mod components;
mod input;
mod knowledge;
mod map_gen;
mod senses;

struct BaseServerPlugin;

impl Plugin for BaseServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntropyPlugin::<WyRand>::default())
            // .insert_resource(FloorGenerationParams::default())
            .add_systems(Startup, generate_test_level)
            // .add_systems(PreUpdate, (update_senses, update_knowledge).chain())
            .add_systems(Update, handle_player_input)
            .add_systems(PostUpdate, send_client_updates);
    }
}

#[derive(Component)]
struct ClientSync;

// todo! this is extremely naive and only updates positions (need to think of more generalized approach)
fn send_client_updates(
    mut client_update_events: EventWriter<ClientUpdate>,
    sync_query: Query<(&SharedId, &EntityKind, &GridPosition), With<ClientSync>>,
) {
    let client_updates: Vec<ClientUpdate> = sync_query
        .iter()
        .map(|(&shared_id, &kind, &GridPosition(pos))| ClientUpdate {
            shared_id,
            kind,
            pos,
        })
        .collect();

    client_update_events.write_batch(client_updates);
}

// Temporary! just generate a single square room
fn generate_test_level(mut commands: Commands) {
    for x in 0..20 {
        for z in 0..20 {
            commands.spawn((
                SharedId::new(),
                ClientSync,
                GridPosition(IVec3::new(x, 0, z)),
                GridShape::SingleBlock,
                Collider,
            ));

            if x == 0 || x == 19 || z == 0 || z == 19 {
                commands.spawn((
                    SharedId::new(),
                    ClientSync,
                    GridPosition(IVec3::new(x, 1, z)),
                    GridShape::SingleBlock,
                    Collider,
                ));

                commands.spawn((
                    SharedId::new(),
                    ClientSync,
                    GridPosition(IVec3::new(x, 2, z)),
                    GridShape::SingleBlock,
                    Collider,
                ));
            }
        }
    }

    commands.spawn((
        SharedId::new(),
        ClientSync,
        PlayerController,
        GridPosition(IVec3::new(10, 1, 10)),
        GridShape::SingleBlock,
        Collider,
        // Sight::new(10),
    ));
}

pub struct LocalServerPlugin;

impl Plugin for LocalServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BaseServerPlugin)
            .add_plugins(ServerSharedIdIndexPlugin::default());
    }
}

#[derive(Component, Copy, Clone, PartialEq, Eq, Hash)]
struct ServerSharedId(SharedId);

type ServerSharedIdIndexPlugin = UniqueComponentIndexPlugin<ServerSharedId, UniqueSparseComponentIndex<ServerSharedId>>;
