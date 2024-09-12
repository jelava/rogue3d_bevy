use std::{collections::HashMap, hash::Hash};

use bevy::{
    input::keyboard::Key,
    math::Vec3,
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
    Walk(Vec3),
}

pub type PlayerInputMap = InputMap<KeyCode, PlayerInputCommand>;

impl Default for PlayerInputMap {
    fn default() -> Self {
        use KeyCode::*;
        use PlayerInputCommand::*;

        Self {
            map: HashMap::from([
                (ArrowUp, Walk(-Vec3::Z)),
                (ArrowDown, Walk(Vec3::Z)),
                (ArrowRight, Walk(Vec3::X)),
                (ArrowLeft, Walk(-Vec3::X)),
            ]),
        }
    }
}
