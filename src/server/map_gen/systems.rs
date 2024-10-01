use bevy::{math::IVec3, prelude::*};
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand::Rng;

use crate::{
    bridge::{BlockSpawned, CreatureSpawned, Id},
    server::{
        components::{Collider, Creature, GridPosition, Name, PlayerController},
        map_gen::{FloorGenerationParams, SimpleRoom},
    },
};

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
    // asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    floor_gen_params: Res<FloorGenerationParams>,
    mut block_spawn_events: EventWriter<BlockSpawned>,
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
                    let id = Id::new();
                    let pos = IVec3::new(x, y, z);

                    commands.spawn((
                        id,
                        GridPosition::SingleBlock(pos),
                        Collider,
                        // PbrBundle {
                        //     mesh: mesh_handle.clone(),
                        //     material: material_handle.clone(),
                        //     transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        //     ..default()
                        // },
                    ));

                    block_spawn_events.send(BlockSpawned { id, pos });
                }
            }
        }
    }
}

pub fn spawn_creatures_in_rooms(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut spawn_events: EventWriter<CreatureSpawned>,
    rooms_query: Query<&SimpleRoom>,
) {
    /*
    let texture_handle = asset_server.load_with_settings(
        "textures/testface.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    let mesh_handle = meshes.add(Rectangle::default());

    let player_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Mask(0.0),
        unlit: true,
        // cull_mode: None,
        // alpha_mode: AlphaMode::Blend,
        // perceptual_roughness: 1.0,
        // reflectance: 0.0,
        ..default()
    });
    */

    /*
    let npc_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        base_color: Color::srgb(1.0, 0.25, 0.25),
        alpha_mode: AlphaMode::Mask(0.0),
        unlit: true,
        // cull_mode: None,
        // alpha_mode: AlphaMode::Blend,
        // perceptual_roughness: 1.0,
        // reflectance: 0.0,
        ..default()
    });
    */

    let mut player_spawned = false;

    for room in &rooms_query {
        if !player_spawned {
            let id = Id::new();

            let spawn_coords = IVec3::new(
                rng.gen_range((room.corner1.x + 1)..room.corner2.x),
                1,
                rng.gen_range((room.corner1.z + 1)..room.corner2.z),
            );

            // let spawn_coords_vec = Vec3::new(
            //     spawn_coords.x as f32,
            //     spawn_coords.y as f32,
            //     spawn_coords.z as f32,
            // );

            commands.spawn((
                id,
                Creature,
                PlayerController,
                Name(String::from("Jessie")),
                GridPosition::SingleBlock(spawn_coords),
                Collider,
                // Billboard,
                // PbrBundle {
                //     mesh: mesh_handle.clone(),
                //     material: player_material_handle.clone(),
                //     transform: Transform::from_translation(spawn_coords_vec),
                //     ..default()
                // },
            ));

            /*
            commands.spawn(Camera3dBundle {
                transform: Transform::from_translation(
                    spawn_coords_vec + Vec3::new(0.0, 8.0, 10.0),
                )
                .looking_at(spawn_coords_vec, Vec3::Y),
                ..default()
            });
            */

            spawn_events.send(CreatureSpawned {
                id,
                pos: spawn_coords,
                is_player: true,
            });

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
