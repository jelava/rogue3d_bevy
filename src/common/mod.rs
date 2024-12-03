/// Components, resources, etc. that are useful for both the client and server
pub mod grid;

use uuid::Uuid;

use bevy::{
    app::Plugin,
    math::IVec3,
    prelude::{Component, Event},
};

pub struct BridgePlugin;

impl Plugin for BridgePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ClientSpawn>();
        app.add_event::<PlayerInputCommand>();
        app.add_event::<PositionUpdate>();
    }
}

// Components for sharing stuff between client and server

/// Client and server entities that are shared should have this component (with the same Uuid if they are the "same"
/// entity). Used to keep track of which client entity corresponds to a server entity when sharing events/data.
#[derive(Component, Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct ShareId(Uuid);

impl ShareId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Component for server entities to indicate status of whether it's shared with the client
// todo: move to server module, client will never need to know about this
#[derive(Component)]
pub enum ClientShare {
    NotYetShared,
    CurrentlyShared,
    PreviouslyShared,
}

impl ClientShare {
    pub fn new() -> Self {
        Self::NotYetShared
    }
}

// client to server events

#[derive(Event, Copy, Clone)]
pub enum PlayerInputCommand {
    Walk(IVec3),
}

// server to client events

/// Sent by server to tell the client to spawn a shared entity
/// todo: this needs to be made more flexible, maybe broken up into separate events for different categories of entities
#[derive(Event)]
pub struct ClientSpawn {
    pub share_id: ShareId,
    pub entity_kind: EntityKind,
}

// todo!!! this is just a temporary hack, will need a much more flexible/extensible way of telling the client what to spawn
pub enum EntityKind {
    Player(IVec3),
    Npc(IVec3),
    Block(IVec3),
}

// #[derive(Event)]
// pub struct RenderInClient;

#[derive(Event)]
pub struct PositionUpdate {
    pub share_id: ShareId,
    pub pos: IVec3,
}
