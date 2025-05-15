use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    client::{components::*, ClientSharedId, ClientSharedIdIndex},
    common::{
        grid::GridPosition, index::unique::UniqueComponentIndex, ClientSyncStart, ClientSyncStop,
        ClientSyncUpdate, EntityKind, SharedId,
    },
};

pub fn client_sync_start_handler(
    mut commands: Commands,
    shared_id_index: Res<ClientSharedIdIndex>,
    temp_asset_handles: Res<TempAssetHandles>,
    mut start_events: EventReader<ClientSyncStart>,
    mut transform_query: Query<&mut Transform, With<ClientSharedId>>,
) {
    for start_event in start_events.read() {
        info!("Client received ClientSyncStart");

        let pos_vec = Vec3::new(
            start_event.pos.x as f32,
            start_event.pos.y as f32,
            start_event.pos.z as f32,
        );

        if let Some(&entity) = shared_id_index.get(&ClientSharedId(start_event.shared_id)) {
            // the entity already exists on client side, update the transform

            // is this actually going to change the transform?
            let mut transform = transform_query.get_mut(entity).unwrap();

            transform.translation = pos_vec;
        } else {
            // no entity with a matching shared ID exists in the client, so spawn it

            match start_event.entity_kind {
                EntityKind::Player => commands.spawn((
                    ClientSharedId(start_event.shared_id),
                    Billboard,
                    Mesh3d(temp_asset_handles.rect_mesh_handle.clone()),
                    MeshMaterial3d(temp_asset_handles.player_material_handle.clone()),
                    Transform::from_translation(pos_vec),
                )),
                EntityKind::Npc => commands.spawn((
                    ClientSharedId(start_event.shared_id),
                    Billboard,
                    Mesh3d(temp_asset_handles.rect_mesh_handle.clone()),
                    MeshMaterial3d(temp_asset_handles.npc_material_handle.clone()),
                    Transform::from_translation(pos_vec),
                )),
                EntityKind::Block => commands.spawn((
                    ClientSharedId(start_event.shared_id),
                    Mesh3d(temp_asset_handles.block_mesh_handle.clone()),
                    MeshMaterial3d(temp_asset_handles.block_material_handle.clone()),
                    Transform::from_translation(pos_vec),
                )),
                EntityKind::Brazier => commands.spawn((
                    ClientSharedId(start_event.shared_id),
                    Billboard,
                    Mesh3d(temp_asset_handles.rect_mesh_handle.clone()),
                    MeshMaterial3d(temp_asset_handles.brazier_material_handle.clone()),
                    PointLight {
                        intensity: 800.0,
                        color: Color::LinearRgba(LinearRgba::new(251.0, 155.0, 114.0, 1.0)),
                        shadows_enabled: true,
                        ..default()
                    },
                    Transform::from_translation(pos_vec),
                )),
            };
        }
    }
}

// todo: need a more generalized way of handling updates
pub fn client_sync_update_handler(
    mut commands: Commands,
    shared_id_index: Res<ClientSharedIdIndex>,
    mut update_events: EventReader<ClientSyncUpdate>,
    mut transform_query: Query<&mut Transform, With<ClientSharedId>>,
) {
    for update_event in update_events.read() {
        // info!("Client received ClientSyncUpdate");

        let pos_vec = Vec3::new(
            update_event.pos.x as f32,
            update_event.pos.y as f32,
            update_event.pos.z as f32,
        );

        if let Some(&entity) = shared_id_index.get(&ClientSharedId(update_event.shared_id)) {
            // is this actually going to change the transform?
            let mut transform = transform_query.get_mut(entity).unwrap();

            transform.translation = pos_vec;
        } else {
            panic!(
                "Could not find client entity with SharedId {:?}",
                update_event.shared_id
            )
        }
    }
}

pub fn client_sync_stop_handler(mut stop_events: EventReader<ClientSyncStop>) {
    for _ in stop_events.read() {
        info!("Client received ClientSyncStop");
        // todo! despawn here? what about cases where entity is not necessarily fully despawned on
        // server but just temporarily not sending updates to client (i.e. entity that is no longer
        // seen by player)
    }
}

// todo: temporary hack. need a better approach than loading everything into a resource at startup eventually
#[derive(Resource)]
pub struct TempAssetHandles {
    block_material_handle: Handle<StandardMaterial>,
    block_mesh_handle: Handle<Mesh>,
    brazier_material_handle: Handle<StandardMaterial>,
    npc_material_handle: Handle<StandardMaterial>,
    player_material_handle: Handle<StandardMaterial>,
    rect_mesh_handle: Handle<Mesh>,
}

pub fn load_temp_asset_handles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rect_mesh_handle = meshes.add(Rectangle::default());

    let player_texture_handle = asset_server.load_with_settings(
        "textures/testrogue.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    let player_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(player_texture_handle.clone()),
        // alpha_mode: AlphaMode::Mask(0.0),
        // unlit: true,
        cull_mode: None,
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default()
    });

    let npc_texture_handle = asset_server.load_with_settings(
        "textures/testgobbo.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    let npc_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(npc_texture_handle.clone()),
        // alpha_mode: AlphaMode::Mask(0.0),
        // unlit: true,
        cull_mode: None,
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default()
    });

    let brazier_texture_handle = asset_server.load_with_settings(
        "textures/testbrazier.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    let brazier_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(brazier_texture_handle.clone()),
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
        // unlit: true,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default()
    });

    commands.insert_resource(TempAssetHandles {
        block_material_handle,
        block_mesh_handle,
        brazier_material_handle,
        npc_material_handle,
        player_material_handle,
        rect_mesh_handle,
    });
}

// misc tech stuff

pub fn update_billboard_transforms(
    camera_transform_query: Query<&Transform, With<Camera>>,
    mut billboards_query: Query<&mut Transform, (With<Billboard>, Without<Camera>)>,
) {
    if let Ok(camera_transform) = camera_transform_query.single() {
        for mut transform in &mut billboards_query {
            transform.look_to(
                camera_transform.forward().normalize() * Vec3::new(1.0, 0.0, 1.0),
                camera_transform.up().normalize(),
            );
        }
    }
}
