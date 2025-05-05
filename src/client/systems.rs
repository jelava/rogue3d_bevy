use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    client::components::*,
    common::{ClientUpdate, EntityKind, SharedId},
};

pub fn handle_client_updates(
    mut commands: Commands,
    mut client_update_events: EventReader<ClientUpdate>,
) {
    for client_update in client_update_events.read() {}
}

/*
pub fn handle_spawns(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client_spawn_events: EventReader<ClientSpawn>,
) {
    use EntityKind::*;

    // todo: load these assets in advance and store handles to them (or anything else more efficient than this...)

    let rect_mesh_handle = meshes.add(Rectangle::default());

    let creature_texture_handle = asset_server.load_with_settings(
        "textures/testface.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    let player_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(creature_texture_handle.clone()),
        alpha_mode: AlphaMode::Mask(0.0),
        unlit: true,
        // cull_mode: None,
        // alpha_mode: AlphaMode::Blend,
        // perceptual_roughness: 1.0,
        // reflectance: 0.0,
        ..default()
    });

    let npc_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(creature_texture_handle.clone()),
        base_color: Color::srgb(1.0, 0.25, 0.25),
        alpha_mode: AlphaMode::Mask(0.0),
        unlit: true,
        // cull_mode: None,
        // alpha_mode: AlphaMode::Blend,
        // perceptual_roughness: 1.0,
        // reflectance: 0.0,
        ..default()
    });

    let block_mesh_handle = meshes.add(Cuboid::default());

    let block_texture_handle = asset_server.load_with_settings(
        "textures/testdots_tiny.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    let block_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(block_texture_handle.clone()),
        unlit: true,
        // perceptual_roughness: 1.0,
        // reflectance: 0.0,
        ..default()
    });

    // let new_npc_bundles = Vec::new();
    // let new_block_bundles = Vec::new();

    for spawn_event in client_spawn_events.read() {
        match spawn_event.entity_kind {
            Player(pos) => {
                let pos_vec = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);

                commands.spawn((
                    spawn_event.share_id,
                    Billboard,
                    Mesh3d(rect_mesh_handle.clone()),
                    MeshMaterial3d(player_material_handle.clone()),
                    Transform::from_translation(pos_vec), /*
                                                          PbrBundle {
                                                              mesh: rect_mesh_handle.clone(),
                                                              material: player_material_handle.clone(),
                                                              transform: Transform::from_translation(pos_vec),
                                                              ..default()
                                                          },
                                                          */
                ));

                // commands.spawn(Camera3d::default());

                /*
                commands.spawn(Camera3dBundle {
                    transform: Transform::from_translation(
                        pos_vec + Vec3::new(0.0, 8.0, 10.0), // todo: hardcoded constant
                    )
                    .looking_at(pos_vec, Vec3::Y),
                    ..default()
                });
                */
            }
            Npc(pos) => {
                let pos_vec = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);

                commands.spawn((
                    spawn_event.share_id,
                    Billboard,
                    Mesh3d(rect_mesh_handle.clone()),
                    MeshMaterial3d(npc_material_handle.clone()),
                    Transform::from_translation(pos_vec), /*
                                                          PbrBundle {
                                                              mesh: rect_mesh_handle.clone(),
                                                              material: npc_material_handle.clone(),
                                                              transform: Transform::from_xyz(pos.x as f32, pos.y as f32, pos.z as f32),
                                                              ..default()
                                                          },
                                                          */
                ));
            }
            Block(pos) => {
                let pos_vec = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);

                commands.spawn((
                    spawn_event.share_id,
                    Mesh3d(block_mesh_handle.clone()),
                    MeshMaterial3d(block_material_handle.clone()),
                    Transform::from_translation(pos_vec), /*
                                                          PbrBundle {
                                                              mesh: block_mesh_handle.clone(),
                                                              material: block_material_handle.clone(),
                                                              transform: Transform::from_xyz(pos.x as f32, pos.y as f32, pos.z as f32),
                                                              ..default()
                                                          },
                                                          */
                ));
            }
        };
    }
}

pub fn handle_position_updates(
    mut position_updates: EventReader<PositionUpdate>,
    mut transform_query: Query<(&mut Transform, &SharedId)>,
) {
    for event in position_updates.read() {
        for (mut transform, transform_id) in &mut transform_query {
            if event.share_id == *transform_id {
                transform.translation =
                    Vec3::new(event.pos.x as f32, event.pos.y as f32, event.pos.z as f32);

                break;
            }
        }
    }
}
*/

// misc tech stuff

pub fn update_billboard_transforms(
    camera_transform_query: Query<&Transform, With<Camera>>,
    mut billboards_query: Query<&mut Transform, (With<Billboard>, Without<Camera>)>,
) {
    if let Ok(camera_transform) = camera_transform_query.single() {
        for mut transform in &mut billboards_query {
            transform.look_to(
                camera_transform.forward().normalize(),
                Vec3::Y, //camera_transform.up().normalize(),
            );
        }
    }
}
