use bevy::{math::IVec3, prelude::Component};

// generic stuff

#[derive(Component)]
pub struct Creature;

// map/block stuff

// #[derive(Component)]
// pub struct Block;

#[derive(Component)]
pub enum GridPosition {
    SingleBlock(IVec3),
    // VerticalLine(i32, i32),
    // HorizontalLine(i32, i32),
    // BoundingBox {
    //     lower_bottom_left: (i32, i32, i32),
    //     dimensions: (i32, i32, i32)
    // }
}

#[derive(Component)]
pub struct Collider;

// player specific stuff

#[derive(Component)]
pub struct PlayerController;

#[derive(Component)]
pub struct Name(pub String);
