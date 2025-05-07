/// Components, resources, etc. that are useful for both the client and server
pub mod grid;
pub mod index;

use index::unique::{UniqueComponentIndexPlugin, UniqueSparseComponentIndex};
use uuid::Uuid;

use bevy::{
    app::Plugin,
    ecs::{
        component::HookContext,
        entity::Entity,
        resource::Resource,
        world::{DeferredWorld, World},
    },
    math::IVec3,
    platform::collections::HashMap,
    prelude::{Component, Event},
};

pub struct BridgePlugin;

impl Plugin for BridgePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ClientUpdate>()
            .add_event::<PlayerInputCommand>();
    }
}

// Components for sharing stuff between client and server

/// Client and server entities that are shared should have this component (with the same Uuid if they are the "same"
/// entity). Used to keep track of which client entity corresponds to a server entity when sharing events/data.
#[derive(Component, Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct SharedId(Uuid);

impl SharedId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

pub type SharedIdIndex = UniqueSparseComponentIndex<SharedId>;

pub type SharedIdIndexPlugin = UniqueComponentIndexPlugin<SharedId, SharedIdIndex>;

// client to server events

#[derive(Event, Copy, Clone)]
pub enum PlayerInputCommand {
    Walk(IVec3),
}

// server to client events

/// Sent by server to tell the client to spawn a shared entity
/// todo: this needs to be made more flexible, maybe broken up into separate events for different categories of entities
/*
#[derive(Event)]
pub struct ClientSpawn {
    pub share_id: SharedId,
    pub entity_kind: EntityKind,
}
*/

#[derive(Component, Copy, Clone)]
// todo!!! this is just a temporary hack, will need a much more flexible/extensible way of telling the client what to spawn
pub enum EntityKind {
    Player(IVec3),
    Npc(IVec3),
    Block(IVec3),
}

// todo: rework basically everything about this lol
#[derive(Event)]
pub struct ClientUpdate {
    pub shared_id: SharedId,
    pub kind: EntityKind,
    pub pos: IVec3,
}
