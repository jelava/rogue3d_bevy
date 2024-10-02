use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use crate::{
    bridge::{BlockSpawned, CreatureSpawned, Id, PositionUpdate},
    client::components::*,
};

pub fn handle_creature_spawns(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut creature_spawn_events: EventReader<CreatureSpawned>,
) {
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

    for spawn_event in creature_spawn_events.read() {
        let spawn_coords = spawn_event.pos;

        let spawn_coords_vec = Vec3::new(
            spawn_coords.x as f32,
            spawn_coords.y as f32,
            spawn_coords.z as f32,
        );

        if spawn_event.is_player {
            commands.spawn((
                spawn_event.id,
                Billboard,
                PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: player_material_handle.clone(),
                    transform: Transform::from_translation(spawn_coords_vec),
                    ..default()
                },
            ));

            commands.spawn(Camera3dBundle {
                transform: Transform::from_translation(
                    spawn_coords_vec + Vec3::new(0.0, 8.0, 10.0),
                )
                .looking_at(spawn_coords_vec, Vec3::Y),
                ..default()
            });
        } else {
            todo!()
        }
    }
}

pub fn handle_block_spawns(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut block_spawn_events: EventReader<BlockSpawned>,
) {
    let texture_handle = asset_server.load_with_settings(
        "textures/testdots_tiny.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    let mesh_handle = meshes.add(Cuboid::default());

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        unlit: true,
        // perceptual_roughness: 1.0,
        // reflectance: 0.0,
        ..default()
    });

    for spawn_event in block_spawn_events.read() {
        commands.spawn((
            spawn_event.id,
            PbrBundle {
                mesh: mesh_handle.clone(),
                material: material_handle.clone(),
                transform: Transform::from_xyz(
                    spawn_event.pos.x as f32,
                    spawn_event.pos.y as f32,
                    spawn_event.pos.z as f32,
                ),
                ..default()
            },
        ));
    }
}

pub fn handle_position_updates(
    mut position_updates: EventReader<PositionUpdate>,
    mut transform_query: Query<(&mut Transform, &Id)>,
) {
    for event in position_updates.read() {
        for (mut transform, transform_id) in &mut transform_query {
            if event.id == *transform_id {
                transform.translation =
                    Vec3::new(event.pos.x as f32, event.pos.y as f32, event.pos.z as f32);

                break;
            }
        }
    }
}

// misc tech stuff

pub fn update_billboard_transforms(
    camera_transform_query: Query<&Transform, With<Camera>>,
    mut billboards_query: Query<&mut Transform, (With<Billboard>, Without<Camera>)>,
) {
    // use std::f32::consts::PI;

    if let Ok(camera_transform) = camera_transform_query.get_single() {
        for mut transform in &mut billboards_query {
            transform.look_to(
                camera_transform.forward().normalize(),
                Vec3::Y, //camera_transform.up().normalize(),
            );
        }
    }
}
