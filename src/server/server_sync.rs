use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

use crate::common::{
    grid::GridPosition,
    index::unique::{UniqueComponentIndexPlugin, UniqueSparseComponentIndex},
    ClientSyncStart, ClientSyncStop, ClientSyncUpdate, EntityKind, SharedId,
};

pub struct LocalServerSyncPlugin;

impl Plugin for LocalServerSyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ServerSharedIdIndexPlugin::default())
            .add_systems(PostUpdate, client_sync_update);
    }
}

#[derive(Component, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ServerSharedId(SharedId);

impl ServerSharedId {
    pub fn new() -> Self {
        Self(SharedId::new())
    }
}

type ServerSharedIdIndexPlugin =
    UniqueComponentIndexPlugin<ServerSharedId, UniqueSparseComponentIndex<ServerSharedId>>;

#[derive(Component)]
#[component(
    on_add = client_sync_start,
    on_remove = client_sync_stop
)]
pub struct ClientSync;

fn client_sync_start(mut world: DeferredWorld, context: HookContext) {
    if let (Some(&ServerSharedId(shared_id)), Some(&entity_kind), Some(&GridPosition(pos))) = (
        world.get(context.entity),
        world.get::<EntityKind>(context.entity),
        world.get(context.entity),
    ) {
        info!("Server sending ClientSyncStart");
        world.send_event(ClientSyncStart {
            shared_id,
            entity_kind,
            pos,
        });
    } else {
        panic!("Added ClientSync to entity with no ServerSharedId and/or EntityKind");
    }
}

fn client_sync_stop(mut world: DeferredWorld, context: HookContext) {
    if let Some(&ServerSharedId(shared_id)) = world.get(context.entity) {
        info!("Server sending ClientSyncStop");
        world.send_event(ClientSyncStop { shared_id });
    } else {
        panic!("Removed ClientSync from entity with no ServerSharedId");
    }
}

// todo! this is extremely naive and only updates positions (need to think of more generalized approach)
fn client_sync_update(
    mut client_update_events: EventWriter<ClientSyncUpdate>,
    sync_query: Query<(&ServerSharedId, &GridPosition), With<ClientSync>>,
) {
    let client_updates: Vec<ClientSyncUpdate> = sync_query
        .iter()
        .map(|(&ServerSharedId(shared_id), &GridPosition(pos))| ClientSyncUpdate { shared_id, pos })
        .collect();

    client_update_events.write_batch(client_updates);
}
