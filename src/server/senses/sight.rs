use std::collections::BinaryHeap;

use bevy::{
    math::IVec3,
    prelude::{Component, Entity, Query, With},
};

use crate::common::{
    grid::{GridPosition, GridShape},
    ClientShare,
};

/// Anything that can be seen (remove to make invisible!)
#[derive(Component)]
pub struct Visible;

/// Anything that blocks visibility of things behind it (remove to make transparent!)
#[derive(Component)]
pub struct Occluder;

/// Component for anything that can see
#[derive(Component)]
pub struct Sight {
    radius: usize,
    // where the "center" of the vision should be relative to the GridPosition of the entity
    origin_offset: IVec3,
    vision_queue: BinaryHeap<SightData>,
}

impl Sight {
    pub fn new(radius: usize) -> Self {
        Self {
            radius,
            origin_offset: IVec3::ZERO,
            vision_queue: BinaryHeap::with_capacity(radius * radius * radius / 4),
        }
    }
}

#[derive(Eq)]
struct SightData {
    pos: IVec3,
    is_seen: bool,
    distance: usize,
    energy: usize,
}

impl PartialEq for SightData {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for SightData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SightData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Rust's BinaryHeap is a max-heap, so the ordering is reversed
        self.distance.cmp(&other.distance).reverse()
    }
}

pub fn update_sight(
    // mut seen_events: EventWriter<EntitySeen>,
    // mut no_longer_seen_events: EventWriter<EntityNoLongerSeen>,
    mut vision_query: Query<(&mut Sight, &GridPosition)>,
    visible_query: Query<
        (Entity, &GridPosition, &GridShape, Option<&mut ClientShare>),
        With<Visible>,
    >,
) {
    for (vision, vision_entity_origin) in &mut vision_query {
        let vision_origin = vision_entity_origin.0 + vision.origin_offset;

        todo!()
    }
}

// todo! currently just checking radius, need to account for occluders
fn test_visibility(pos1: IVec3, pos2: IVec3, range: usize) -> bool {
    let p = pos2 - pos1;
    return p.x * p.x + p.y * p.y + p.z * p.z <= (range * range) as i32;
}
