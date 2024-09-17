use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use crate::components::*;

// startup/initialization

pub fn generate_blocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let texture_handle = asset_server.load_with_settings(
    //     "textures/testgrid.png",
    //     |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    // );

    let texture_handle = asset_server.load("textures/testgrid_big.png");

    let mesh_handle = meshes.add(Cuboid::default());

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        unlit: true,
        // perceptual_roughness: 1.0,
        // reflectance: 0.0,
        ..default()
    });

    for x in -10..11 {
        for z in -10..11 {
            commands.spawn((
                GridPosition::SingleBlock(IVec3::new(x, -1, z)),
                Collider,
                PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: Transform::from_xyz(x as f32, -1.0, z as f32),
                    ..default()
                },
            ));
        }
    }

    for z in -3..4 {
        commands.spawn((
            GridPosition::SingleBlock(IVec3::new(-3, 0, z)),
            Collider,
            PbrBundle {
                mesh: mesh_handle.clone(),
                material: material_handle.clone(),
                transform: Transform::from_xyz(-3.0, 0.0, z as f32),
                ..default()
            },
        ));
    }

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.5, 2.5, 3.5),
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

pub fn spawn_creatures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load_with_settings(
        "textures/testface.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    let mesh_handle = meshes.add(Rectangle::default());

    let player_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        unlit: true,
        // cull_mode: None,
        // alpha_mode: AlphaMode::Blend,
        // perceptual_roughness: 1.0,
        // reflectance: 0.0,
        ..default()
    });

    let npc_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        base_color: Color::srgb(1.0, 0.25, 0.25),
        unlit: true,
        // cull_mode: None,
        // alpha_mode: AlphaMode::Blend,
        // perceptual_roughness: 1.0,
        // reflectance: 0.0,
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn((
        Creature,
        PlayerController,
        Name(String::from("Jessie")),
        GridPosition::SingleBlock(IVec3::new(0, 0, 0)),
        Collider,
        // Billboard,
        PbrBundle {
            mesh: mesh_handle.clone(),
            material: player_material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        Creature,
        Name(String::from("Gnorm the gnome")),
        GridPosition::SingleBlock(IVec3::new(5, 0, -2)),
        Collider,
        // Billboard,
        PbrBundle {
            mesh: mesh_handle.clone(),
            material: npc_material_handle.clone(),
            transform: Transform::from_xyz(5.0, 0.0, -2.0),
            ..default()
        },
    ));
}

// misc tech stuff

pub fn update_camera(
    mut camera_transform_query: Query<&mut Transform, With<Camera>>,
    player_transform_query: Query<&Transform, (With<PlayerController>, Without<Camera>)>,
) {
    if let (Ok(mut camera_transform), Ok(player_transform)) = (
        camera_transform_query.get_single_mut(),
        player_transform_query.get_single(),
    ) {
        // TODO: don't hardcode camera offset, come up with better approach to handling camera in general
        camera_transform.translation = player_transform.translation + Vec3::new(0.0, 5.0, 10.0);
    }
}

pub fn update_billboard_transforms(
    camera_transform_query: Query<&Transform, With<Camera>>,
    mut billboards_query: Query<&mut Transform, (With<Billboard>, Without<Camera>)>,
) {
    // use std::f32::consts::PI;

    if let Ok(camera_transform) = camera_transform_query.get_single() {
        for mut transform in &mut billboards_query {
            transform.look_to(camera_transform.forward(), camera_transform.up());
        }
    }
}
