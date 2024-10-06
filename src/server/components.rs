use bevy::{math::IVec3, prelude::Component};

// generic stuff

#[derive(Component)]
pub struct Creature;

// map/block stuff

// #[derive(Component)]
// pub struct Block;

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

#[derive(Component)]
pub struct Name(pub String);
