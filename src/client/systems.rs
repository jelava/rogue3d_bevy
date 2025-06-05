use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::client::components::Billboard;

// todo: temporary hack. need a better approach than loading everything into a resource at startup eventually
#[derive(Resource)]
pub struct TempAssetHandles {
    pub block_material_handle: Handle<StandardMaterial>,
    pub block_mesh_handle: Handle<Mesh>,
    pub brazier_material_handle: Handle<StandardMaterial>,
    pub npc_material_handle: Handle<StandardMaterial>,
    pub player_material_handle: Handle<StandardMaterial>,
    pub rect_mesh_handle: Handle<Mesh>,
    pub unsynced_block_material_handle: Handle<StandardMaterial>,
    pub unsynced_brazier_material_handle: Handle<StandardMaterial>,
    pub unsynced_npc_material_handle: Handle<StandardMaterial>,
    pub unsynced_player_material_handle: Handle<StandardMaterial>,
}

pub fn load_temp_asset_handles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_texture_handle = asset_server.load_with_settings(
        "textures/testrogue.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    let npc_texture_handle = asset_server.load_with_settings(
        "textures/testgobbo.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    let brazier_texture_handle = asset_server.load_with_settings(
        "textures/testbrazier.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    let block_texture_handle = asset_server.load_with_settings(
        "textures/testdots_tiny.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    let unsynced_base_color = Color::linear_rgb(0.2, 0.4, 0.4);

    commands.insert_resource(TempAssetHandles {
        block_material_handle: materials.add(StandardMaterial {
            base_color_texture: Some(block_texture_handle.clone()),
            // unlit: true,
            perceptual_roughness: 1.0,
            reflectance: 0.0,
            ..default()
        }),
        block_mesh_handle: meshes.add(Cuboid::default()),
        brazier_material_handle: materials.add(StandardMaterial {
            base_color_texture: Some(brazier_texture_handle.clone()),
            alpha_mode: AlphaMode::Mask(0.0),
            unlit: true,
            // cull_mode: None,
            // alpha_mode: AlphaMode::Blend,
            // perceptual_roughness: 1.0,
            // reflectance: 0.0,
            ..default()
        }),
        npc_material_handle: materials.add(StandardMaterial {
            base_color_texture: Some(npc_texture_handle.clone()),
            // alpha_mode: AlphaMode::Mask(0.0),
            // unlit: true,
            cull_mode: None,
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 1.0,
            reflectance: 0.0,
            ..default()
        }),
        player_material_handle: materials.add(StandardMaterial {
            base_color_texture: Some(player_texture_handle.clone()),
            // alpha_mode: AlphaMode::Mask(0.0),
            // unlit: true,
            cull_mode: None,
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 1.0,
            reflectance: 0.0,
            ..default()
        }),
        rect_mesh_handle: meshes.add(Rectangle::default()),
        unsynced_block_material_handle: materials.add(StandardMaterial {
            base_color: unsynced_base_color,
            base_color_texture: Some(block_texture_handle.clone()),
            // unlit: true,
            perceptual_roughness: 1.0,
            reflectance: 0.0,
            ..default()
        }),
        unsynced_brazier_material_handle: materials.add(StandardMaterial {
            base_color: unsynced_base_color,
            base_color_texture: Some(brazier_texture_handle.clone()),
            alpha_mode: AlphaMode::Mask(0.0),
            unlit: true,
            // cull_mode: None,
            // alpha_mode: AlphaMode::Blend,
            // perceptual_roughness: 1.0,
            // reflectance: 0.0,
            ..default()
        }),
        unsynced_npc_material_handle: materials.add(StandardMaterial {
            base_color: unsynced_base_color,
            base_color_texture: Some(npc_texture_handle.clone()),
            // alpha_mode: AlphaMode::Mask(0.0),
            // unlit: true,
            cull_mode: None,
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 1.0,
            reflectance: 0.0,
            ..default()
        }),
        unsynced_player_material_handle: materials.add(StandardMaterial {
            base_color: unsynced_base_color,
            base_color_texture: Some(player_texture_handle.clone()),
            // alpha_mode: AlphaMode::Mask(0.0),
            // unlit: true,
            cull_mode: None,
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 1.0,
            reflectance: 0.0,
            ..default()
        }),
    });
}

// misc graphical stuff

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
