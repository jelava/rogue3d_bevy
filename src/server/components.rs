use bevy::{math::IVec3, prelude::Component};

// map/block stuff

#[derive(Component)]
pub struct GridPosition(pub IVec3);

#[derive(Component)]
pub enum GridShape {
    SingleBlock,
}

#[derive(Component)]
pub struct Collider;

// player specific stuff

#[derive(Component)]
pub struct PlayerController;
