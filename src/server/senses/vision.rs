use std::collections::HashSet;

use bevy::{
    math::IVec3,
    prelude::{Commands, Component, Entity, Query, With, Without},
};

use crate::{
    bridge::{ClientSync, Id},
    server::components::{GridPosition, GridShape},
};

/// Anything that can be seen (remove to make invisible!)
#[derive(Component)]
pub struct Visible;

/// Anything that blocks visibility of things behind it (remove to make transparent!)
#[derive(Component)]
pub struct Occluder;

/// Component for anything that can see
#[derive(Component)]
pub struct Vision {
    pub show_to_client: bool,
    pub range: usize,
    pub origin_offset: IVec3,
    pub seen_entities: HashSet<Entity>, // vision_data: Option<LocalGridData<VisionData>>
}

impl Default for Vision {
    fn default() -> Self {
        // todo: hardcoded constants :(
        Self {
            show_to_client: false,
            range: 10,
            origin_offset: IVec3::ZERO,
            seen_entities: HashSet::with_capacity(2 * 2 * 2),
        }
    }
}

// struct VisionData {
//     is_visible: bool,
//     distance: usize
// }

/// Localized cubic slice of the grid containing per-block data
// pub struct LocalGridData<T> {
//     bounds: (IVec3, IVec3),
//     data: Vec<T>
// }

// todo? querying and then checking for pos change is a bit inefficient, if it becomes a  consider switching to observers or something else
pub fn update_vision(
    mut commands: Commands,
    mut vision_query: Query<(&mut Vision, &GridPosition)>,
    visible_query: Query<(Entity, &GridPosition, &GridShape), (With<Visible>, Without<ClientSync>)>,
) {
    for (mut vision, vision_entity_origin) in &mut vision_query {
        let vision_origin = vision_entity_origin.0 + vision.origin_offset;

        for (visible_entity, visible_entity_origin, visible_entity_shape) in &visible_query {
            // todo: "use" the visible_entity_shape (for now that is trivial)

            let seen = test_visibility(vision_origin, visible_entity_origin.0, vision.range);
            let previously_seen = vision.seen_entities.contains(&visible_entity);

            if seen && !previously_seen {
                // entity is being seen for the first time, add to seen_entities (and start syncing info about it with client)
                vision.seen_entities.insert(visible_entity);

                if vision.show_to_client {
                    commands.entity(visible_entity).insert(ClientSync);
                }
            } else if !seen && previously_seen {
                // entity was seen last time vision was updated but is not seen this time, so remove it from seen_entities
                vision.seen_entities.remove(&visible_entity);

                if vision.show_to_client {
                    commands.entity(visible_entity).remove::<ClientSync>();
                }
            }
        }
    }
}

// todo: currently just checking radius, need to account for occluders
fn test_visibility(pos1: IVec3, pos2: IVec3, range: usize) -> bool {
    let p = pos2 - pos1;
    return p.x * p.x + p.y * p.y + p.z * p.z <= (range * range) as i32;
}
