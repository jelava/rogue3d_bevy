use bevy::prelude::*;

use crate::{
    bridge::{Id, PlayerInputCommand, PositionUpdate},
    server::components::{Collider, GridPosition, PlayerController},
};

pub fn handle_player_input(
    mut player_input_commands: EventReader<PlayerInputCommand>,
    mut position_updates: EventWriter<PositionUpdate>,
    mut player_position_query: Query<
        (&mut GridPosition, &Id),
        (With<PlayerController>, With<Collider>),
    >,
    colliders_query: Query<&GridPosition, (With<Collider>, Without<PlayerController>)>,
) {
    use PlayerInputCommand::*;

    let (mut player_grid_pos, id) = player_position_query.single_mut();

    if let Some(command) = player_input_commands.read().next() {
        match *command {
            Walk(dir) => match *player_grid_pos {
                GridPosition::SingleBlock(pos) => {
                    let updated_pos = pos + dir;

                    if is_block_unoccupied(updated_pos, colliders_query) {
                        *player_grid_pos = GridPosition::SingleBlock(updated_pos);

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
    colliders_query: Query<&GridPosition, (With<Collider>, Without<PlayerController>)>,
) -> bool {
    for collider in &colliders_query {
        match collider {
            GridPosition::SingleBlock(collider_pos) => {
                if pos == *collider_pos {
                    return false;
                }
            }
        }
    }

    true
}
