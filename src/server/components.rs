use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

use crate::common::{grid::GridPosition, ClientSyncStart, ClientSyncStop, EntityKind};

use super::ServerSharedId;

// map/block stuff

#[derive(Component)]
pub struct Collider;

// player specific stuff

#[derive(Component)]
pub struct PlayerController;

// client communication stuff

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
