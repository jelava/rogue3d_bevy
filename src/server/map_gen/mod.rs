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
            floor_size: IVec3::new(100, 3, 100),
            min_room_size: IVec3::new(5, 3, 5),
            max_room_size: IVec3::new(30, 3, 30),
            max_num_rooms: 50,
            // max_room_attempts: 3,
        }
    }
}

#[derive(Component)]
pub struct SimpleRoom {
    corner1: IVec3,
    corner2: IVec3,
}
