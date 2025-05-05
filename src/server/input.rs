use bevy::prelude::*;

use crate::{
    common::{
        grid::{GridPosition, GridShape, SparseGridIndex},
        index::ComponentIndex,
        PlayerInputCommand,
    },
    server::components::{Collider, PlayerController},
};

pub fn handle_player_input(
    mut player_input_commands: EventReader<PlayerInputCommand>,
    grid_index: Res<SparseGridIndex>,
    mut player_position_query: Query<
        (&mut GridPosition, &GridShape),
        (With<PlayerController>, With<Collider>),
    >,
    colliders_query: Query<
        (&GridPosition, &GridShape),
        (With<Collider>, Without<PlayerController>),
    >,
) -> Result {
    use PlayerInputCommand::*;

    let (mut player_pos, player_shape) = player_position_query.single_mut()?;

    if let Some(command) = player_input_commands.read().next() {
        match *command {
            Walk(dir) => match *player_shape {
                GridShape::SingleBlock => {
                    // todo? .0 is kinda ugly, use destructuring or something?
                    let updated_pos = GridPosition(player_pos.0 + dir);

                    // todo! this is very simplistic/naive, simply doesn't allow 2+ entities in same cell regardless of whether they have Collider component
                    if grid_index.get(&updated_pos).is_none() {
                        *player_pos = updated_pos;
                        info!("player moved to {:?}", updated_pos.0);
                    }
                }
            },
        }
    }

    Ok(())
}
