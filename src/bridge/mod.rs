/// Events (and eventually maybe other stuff?) that are used for communication between the client and server
use bevy::{
    app::Plugin,
    math::IVec3,
    prelude::{Component, Event},
};
use uuid::Uuid;

pub struct BridgePlugin;

impl Plugin for BridgePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<BlockSpawned>();
        app.add_event::<CreatureSpawned>();
        app.add_event::<PlayerInputCommand>();
        app.add_event::<PositionUpdate>();
    }
}

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Event, Copy, Clone)]
pub enum PlayerInputCommand {
    Walk(IVec3),
}

#[derive(Event)]
pub struct CreatureSpawned {
    pub id: Id,
    pub pos: IVec3,
    pub is_player: bool,
}

// todo - implement an actual chunking system and don't deal with blocks 1 by 1
#[derive(Event)]
pub struct BlockSpawned {
    pub id: Id,
    pub pos: IVec3,
}

#[derive(Event)]
pub struct PositionUpdate {
    pub id: Id,
    pub pos: IVec3,
}
