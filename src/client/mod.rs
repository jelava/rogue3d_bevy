mod components;
mod input;
mod palettization;
mod systems;

use std::f32::consts::PI;

use bevy::{
    app::{Plugin, PreUpdate, Startup, Update},
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    window::{Window, WindowPlugin},
};
use palettization::{PalettizationEffect, PalettizationPlugin};

use crate::{
    client::{
        input::{systems::*, PlayerInputMap},
        systems::*,
    },
    common::{
        index::unique::{UniqueComponentIndexPlugin, UniqueSparseComponentIndex},
        SharedId,
    },
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

        app.add_plugins(default_plugins)
            .add_plugins(PalettizationPlugin)
            .init_resource::<PlayerInputMap>()
            // .add_systems(PreStartup, load_temp_asset_handles)
            .add_systems(Startup, (load_temp_asset_handles, spawn_camera))
            .add_systems(PreUpdate, client_sync_update_handler)
            .add_systems(
                Update,
                (
                    player_kb_input_mapper,
                    (
                        client_sync_start_handler,
                        client_sync_update_handler,
                        client_sync_stop_handler,
                    )
                        .chain(),
                    (handle_camera_input, update_billboard_transforms).chain(),
                ),
            );
    }
}

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    let lut_image = asset_server.load_with_settings(
        "textures/palette-luts/unweighted.png",
        |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::nearest(),
    );

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(10.0, 5.0, 20.0))
            .looking_at(Vec3::new(10.0, 1.0, 10.0), Dir3::Y),
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
        app.add_plugins(BaseClientPlugin)
            .add_plugins(ClientSharedIdIndexPlugin::default());
    }
}

#[derive(Component, Copy, Clone, PartialEq, Eq, Hash)]
struct ClientSharedId(SharedId);

impl From<SharedId> for ClientSharedId {
    fn from(value: SharedId) -> Self {
        Self(value)
    }
}

type ClientSharedIdIndex = UniqueSparseComponentIndex<ClientSharedId>;

type ClientSharedIdIndexPlugin = UniqueComponentIndexPlugin<ClientSharedId, ClientSharedIdIndex>;
