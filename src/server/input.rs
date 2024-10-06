use bevy::prelude::*;

use crate::{
    bridge::{Id, PlayerInputCommand, PositionUpdate},
    server::components::{Collider, GridPosition, PlayerController},
};

use super::components::GridShape;

pub fn handle_player_input(
    mut player_input_commands: EventReader<PlayerInputCommand>,
    mut position_updates: EventWriter<PositionUpdate>,
    mut player_position_query: Query<
        (&mut GridPosition, &GridShape, &Id),
        (With<PlayerController>, With<Collider>),
    >,
    colliders_query: Query<
        (&GridPosition, &GridShape),
        (With<Collider>, Without<PlayerController>),
    >,
) {
    use PlayerInputCommand::*;

    let (mut player_pos, player_shape, id) = player_position_query.single_mut();

    if let Some(command) = player_input_commands.read().next() {
        match *command {
            Walk(dir) => match *player_shape {
                GridShape::SingleBlock => {
                    // TODO: .0 is kinda ugly, use destructuring or something?
                    let updated_pos = player_pos.0 + dir;

                    if is_block_unoccupied(updated_pos, colliders_query) {
                        *player_pos = GridPosition(updated_pos);

                        position_updates.send(PositionUpdate {
                            id: *id,
                            pos: updated_pos,
                        });
                    }
                }
            },
        }
    }
}

fn is_block_unoccupied(
    pos: IVec3,
    colliders_query: Query<
        (&GridPosition, &GridShape),
        (With<Collider>, Without<PlayerController>),
    >,
) -> bool {
    for (GridPosition(collider_pos), collider_shape) in &colliders_query {
        match collider_shape {
            GridShape::SingleBlock => {
                if pos == *collider_pos {
                    return false;
                }
            }
        }
    }

    true
}
