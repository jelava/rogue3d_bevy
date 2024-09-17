use std::{collections::HashMap, hash::Hash};

use bevy::{
    math::IVec3,
    prelude::{Event, KeyCode, Resource},
};

pub mod systems;

#[derive(Resource)]
pub struct InputMap<I: Eq + Hash, C> {
    map: HashMap<I, C>,
}

impl<I: Eq + Hash, C> InputMap<I, C> {
    pub fn get(&self, input: &I) -> Option<&C> {
        self.map.get(input)
    }
}

#[derive(Event, Copy, Clone)]
pub enum PlayerInputCommand {
    Walk(IVec3),
}

pub type PlayerInputMap = InputMap<KeyCode, PlayerInputCommand>;

impl Default for PlayerInputMap {
    fn default() -> Self {
        use KeyCode::*;
        use PlayerInputCommand::*;

        Self {
            map: HashMap::from([
                (ArrowUp, Walk(-IVec3::Z)),
                (ArrowDown, Walk(IVec3::Z)),
                (ArrowRight, Walk(IVec3::X)),
                (ArrowLeft, Walk(-IVec3::X)),
            ]),
        }
    }
}

// pub type CameraInputMap = InputMap<>
