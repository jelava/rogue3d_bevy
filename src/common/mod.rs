/// Components, resources, etc. that are useful for both the client and server
pub mod grid;
pub mod index;

use grid::SparseGridIndexPlugin;
use index::unique::{UniqueComponentIndexPlugin, UniqueSparseComponentIndex};
use uuid::Uuid;

use bevy::{app::Plugin, math::IVec3, prelude::*};

struct BaseBridgePlugin;

impl Plugin for BaseBridgePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ClientSyncStart>()
            .add_event::<ClientSyncUpdate>()
            .add_event::<ClientSyncStop>()
            .add_event::<PlayerInputCommand>();
    }
}

pub struct LocalBridgePlugin;

impl Plugin for LocalBridgePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(BaseBridgePlugin)
            .add_plugins(SparseGridIndexPlugin::default());
    }
}

// Components for sharing stuff between client and server

// todo! update comments - this is not itself a component, just wrapped by components
/// Client and server entities that are shared should have this component (with the same Uuid if they are the "same"
/// entity). Used to keep track of which client entity corresponds to a server entity when sharing events/data.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct SharedId(Uuid);

impl SharedId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

// pub type SharedIdIndex = UniqueSparseComponentIndex<SharedId>;

// pub type SharedIdIndexPlugin = UniqueComponentIndexPlugin<SharedId, SharedIdIndex>;

// client to server events

#[derive(Event, Copy, Clone)]
pub enum PlayerInputCommand {
    Walk(IVec3),
}

// server to client events

// todo: rework
/// Event fired when the ClientSync component is added to a server entity and synchronization begins
/// If nothing with a matching SharedId exists then the client will spawn a new entity, otherwise it
/// will be treated like an update to an existing entity.
#[derive(Event)]
pub struct ClientSyncStart {
    pub shared_id: SharedId,
    pub entity_kind: EntityKind,
    pub pos: IVec3,
}

// todo: rework
/// Event fired regularly for all server entities with ClientSync component to send state updates
/// to client.
#[derive(Event)]
pub struct ClientSyncUpdate {
    pub shared_id: SharedId,
    // pub kind: EntityKind,
    pub pos: IVec3,
}

// todo: rework
/// Event fired when the ClientSync component is removed from a server entity. (todo: better docs here...)
#[derive(Event)]
pub struct ClientSyncStop {
    pub shared_id: SharedId,
}

// todo!!! this is just a temporary hack, will need a much more flexible/extensible way of telling the client what to spawn
#[derive(Component, Copy, Clone)]
pub enum EntityKind {
    Player,
    Npc,
    Block,
}
