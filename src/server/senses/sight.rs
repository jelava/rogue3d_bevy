use std::collections::BinaryHeap;

use bevy::{ecs::query::Has, math::IVec3, platform::collections::HashSet, prelude::*};

use crate::{
    common::{
        grid::{GridPosition, SparseGridIndex},
        index::ComponentIndex,
    },
    server::server_sync::ClientSync,
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
    range: i32, // would make more sense to be unsigned but makes arithmetic with IVec3 components simpler as i32
    // where the "center" of the vision should be relative to the GridPosition of the entity
    origin_offset: IVec3,
    // vision_queue: BinaryHeap<SightData>,
    visible_cells: HashSet<IVec3>,
}

/// For entities whose sight should determine what gets shared with client
#[derive(Component)]
pub struct ClientSyncSight;

impl Sight {
    pub fn new(range: i32) -> Self {
        Self {
            range,
            origin_offset: IVec3::ZERO,
            // vision_queue: BinaryHeap::with_capacity(radius * radius * radius / 4),
            visible_cells: HashSet::with_capacity((range * range * range / 4) as usize),
        }
    }
}

// #[derive(Eq)]
// struct SightData {
//     pos: IVec3,
//     is_seen: bool,
//     distance: usize,
//     energy: usize,
// }

// impl PartialEq for SightData {
//     fn eq(&self, other: &Self) -> bool {
//         self.distance == other.distance
//     }
// }

// impl PartialOrd for SightData {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for SightData {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         // Rust's BinaryHeap is a max-heap, so the ordering is reversed
//         self.distance.cmp(&other.distance).reverse()
//     }
// }

pub fn update_sight(
    // mut seen_events: EventWriter<EntitySeen>,
    // mut no_longer_seen_events: EventWriter<EntityNoLongerSeen>,
    mut commands: Commands,
    grid_index: Res<SparseGridIndex>,
    mut sight_query: Query<(&mut Sight, Has<ClientSyncSight>, &GridPosition)>,
    // visible_query: Query<
    //     (Entity, &GridPosition, &GridShape), //, Option<&mut ClientShare>),
    //     With<Visible>,
    // >,
) {
    // todo! this is even less efficient!
    let mut old_client_visible_cells = HashSet::new();
    let mut new_client_visible_cells = HashSet::new();

    for (mut sight, sync_sight, &GridPosition(sight_entity_pos)) in &mut sight_query {
        //sight.visible_cells.clear();

        // todo: this is probably not too efficient
        let mut new_visible_cells = HashSet::new();

        let sight_origin = sight_entity_pos + sight.origin_offset;

        for x in (sight_origin.x - sight.range)..=(sight_origin.x + sight.range) {
            for y in (sight_origin.y - sight.range)..=(sight_origin.y + sight.range) {
                for z in (sight_origin.z - sight.range)..=(sight_origin.z + sight.range) {
                    let pos = IVec3::new(x, y, z);
                    let was_visible = sight.visible_cells.contains(&pos);
                    let is_visible = test_cell_visibility(sight_origin, pos, sight.range);

                    // if !was_visible && is_visible {
                    if is_visible {
                        new_visible_cells.insert(pos);

                        if sync_sight {
                            new_client_visible_cells.insert(pos);

                            // check if there are entities at pos and add ClientSync to them
                            // if let Some(entities) = grid_index.get(&GridPosition(pos)) {
                            //     for &entity in entities {
                            //         // info!("sight adding ClientSync");
                            //         commands.entity(entity).insert(ClientSync);
                            //     }
                            // }
                        }
                    } /*else if was_visible && !is_visible {
                          sight.visible_cells.remove(&pos);

                          if sync_sight {
                              if let Some(entities) = grid_index.get(&GridPosition(pos)) {
                                  for &entity in entities {
                                      info!("sight removing ClientSync");
                                      commands.entity(entity)
                                          .remove::<ClientSync>();
                                  }
                              }
                          }
                      } else {
                          info!("was: {:?}, is: {:?}", was_visible, is_visible);
                      }*/

                    if sync_sight && was_visible {
                        old_client_visible_cells.insert(pos);
                    }
                }
            }
        }

        // remove ClientSync from no longer visible cells
        for &pos in sight.visible_cells.difference(&new_visible_cells) {
            // if let Some(entities) = grid_index.get(&GridPosition(pos)) {
            //     for &entity in entities {
            //         info!("sight removing ClientSync");
            //         commands.entity(entity).remove::<ClientSync>();
            //     }
            // }

            if sync_sight {
                old_client_visible_cells.insert(pos);
            }
        }

        sight.visible_cells = new_visible_cells;
    }

    // todo: this is all hacky and inefficient
    for &pos in new_client_visible_cells.difference(&old_client_visible_cells) {
        if let Some(entities) = grid_index.get(&GridPosition(pos)) {
            for &entity in entities {
                // info!("sight adding ClientSync");
                commands.entity(entity).insert(ClientSync);
            }
        }
    }

    for &pos in old_client_visible_cells.difference(&new_client_visible_cells) {
        if let Some(entities) = grid_index.get(&GridPosition(pos)) {
            for &entity in entities {
                // info!("sight adding ClientSync");
                commands.entity(entity).remove::<ClientSync>();
            }
        }
    }
}

// todo! currently just checking radius, need to account for occluders
fn test_cell_visibility(pos1: IVec3, pos2: IVec3, range: i32) -> bool {
    let p = pos2 - pos1;
    // return p.x <= range && p.y <= range && p.y <= range;
    return p.x * p.x + p.y * p.y + p.z * p.z <= range * range;
}
