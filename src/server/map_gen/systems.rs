use bevy::{math::IVec3, prelude::*};
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand::Rng;

use crate::{bridge::{ClientShare, ShareId}, server::{
    components::{Collider, GridPosition, GridShape, PlayerController},
    map_gen::{FloorGenerationParams, SimpleRoom},
    senses::vision::Vision,
}};

pub fn generate_abstract_floor(
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    floor_gen_params: Res<FloorGenerationParams>,
) {
    for _ in 0..floor_gen_params.max_num_rooms {
        let corner1 = IVec3::new(
            rng.gen_range(0..(floor_gen_params.floor_size.x - floor_gen_params.max_room_size.x)),
            0, //rng.gen_range(0..params.floor_size.y),
            rng.gen_range(0..(floor_gen_params.floor_size.z - floor_gen_params.max_room_size.z)),
        );

        let corner2 = corner1
            + IVec3::new(
                rng.gen_range(floor_gen_params.min_room_size.x..=floor_gen_params.max_room_size.x),
                floor_gen_params.max_room_size.y, //rng.gen_range(params.min_room_size.y..=params.max_room_size.y),
                rng.gen_range(floor_gen_params.min_room_size.z..=floor_gen_params.max_room_size.z),
            );

        commands.spawn(SimpleRoom { corner1, corner2 });
    }
}

pub fn generate_blocks_from_rooms(
    mut commands: Commands,
    floor_gen_params: Res<FloorGenerationParams>,
    rooms_query: Query<&SimpleRoom>,
) {
    for x in 0..floor_gen_params.floor_size.x {
        for y in 0..floor_gen_params.floor_size.y {
            for z in 0..floor_gen_params.floor_size.z {
                let mut inside_room = false;

                for room in &rooms_query {
                    if x > room.corner1.x
                        && x < room.corner2.x
                        && y > room.corner1.y
                        && y < room.corner2.y
                        && z > room.corner1.z
                        && z < room.corner2.z
                    {
                        inside_room = true;
                        break;
                    }
                }

                if !inside_room {
                    let pos = IVec3::new(x, y, z);

                    commands.spawn((GridPosition(pos), GridShape::SingleBlock, Collider));
                }
            }
        }
    }
}

pub fn spawn_creatures_in_rooms(
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    rooms_query: Query<&SimpleRoom>,
) {
    let mut player_spawned = false;

    for room in &rooms_query {
        if !player_spawned {
            let spawn_coords = IVec3::new(
                rng.gen_range((room.corner1.x + 1)..room.corner2.x),
                1,
                rng.gen_range((room.corner1.z + 1)..room.corner2.z),
            );

            commands.spawn((
                ShareId::new(),
                ClientShare::new(),
                PlayerController,
                GridPosition(spawn_coords),
                GridShape::SingleBlock,
                Collider,
                Vision {
                    share_with_client: true,
                    range: 20,
                    ..default()
                },
            ));

            player_spawned = true;
        }

        /*
        let creatures_in_room = rng.gen_range(0..=1);

        for _ in 0..creatures_in_room {
            let spawn_coords = IVec3::new(
                rng.gen_range((room.corner1.x + 1)..room.corner2.x),
                1,
                rng.gen_range((room.corner1.z + 1)..room.corner2.z),
            );

            let spawn_coords_vec = Vec3::new(
                spawn_coords.x as f32,
                spawn_coords.y as f32,
                spawn_coords.z as f32,
            );

            commands.spawn((
                Creature,
                Name(String::from("NPC")),
                GridPosition::SingleBlock(spawn_coords),
                Collider,
                Billboard,
                PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: npc_material_handle.clone(),
                    transform: Transform::from_translation(spawn_coords_vec),
                    ..default()
                },
            ));
        }
        */
    }
}
