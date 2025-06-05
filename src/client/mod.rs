pub mod client_sync;
mod components;
mod input;
mod palettization;
mod systems;

use bevy::{
    app::{Plugin, Startup, Update},
    core_pipeline::tonemapping::Tonemapping,
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    window::{Window, WindowPlugin},
};

use crate::client::{
    client_sync::LocalClientSyncPlugin,
    input::{systems::*, PlayerInputMap},
    palettization::{PalettizationEffect, PalettizationPlugin},
    systems::*,
};

// The shared baseline for both the local and (eventually) networked version of the client plugin
struct BaseClientPlugin;

impl Plugin for BaseClientPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "rogue3d".into(),
                ..default()
            }),
            ..default()
        });

        app.add_plugins((default_plugins, PalettizationPlugin))
            .init_resource::<PlayerInputMap>()
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    player_kb_input_mapper,
                    (handle_camera_input, update_billboard_transforms).chain(),
                ),
            );
    }
}

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    let lut_image = asset_server.load_with_settings(
        "textures/palette-luts/converted.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
            // settings.is_srgb = false;
        },
    );

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(10.0, 5.0, 20.0))
            .looking_at(Vec3::new(10.0, 1.0, 10.0), Dir3::Y),
        Tonemapping::None,
        PalettizationEffect { lut_image },
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_MOON_NIGHT,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_translation(Vec3::new(1.0, 5.0, 1.0)).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}

pub struct LocalClientPlugin;

impl Plugin for LocalClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((BaseClientPlugin, LocalClientSyncPlugin));
    }
}
