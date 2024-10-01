use bevy::{
    math::IVec3,
    prelude::{Component, Resource},
};

pub mod systems;

#[derive(Resource)]
pub struct FloorGenerationParams {
    floor_size: IVec3,
    min_room_size: IVec3,
    max_room_size: IVec3,
    max_num_rooms: usize,
    // max_room_attempts: usize,
}

impl Default for FloorGenerationParams {
    fn default() -> Self {
        Self {
            floor_size: IVec3::new(30, 3, 30),
            min_room_size: IVec3::new(3, 3, 3),
            max_room_size: IVec3::new(10, 3, 10),
            max_num_rooms: 25,
            // max_room_attempts: 3,
        }
    }
}

#[derive(Component)]
pub struct SimpleRoom {
    corner1: IVec3,
    corner2: IVec3,
}
