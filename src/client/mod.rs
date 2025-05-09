mod components;
mod input;
mod systems;

use bevy::{
    app::{Plugin, PreUpdate, Startup, Update},
    prelude::*,
    window::{Window, WindowPlugin},
};

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
            .init_resource::<PlayerInputMap>()
            .add_systems(PreStartup, load_temp_asset_handles)
            .add_systems(Startup, spawn_camera)
            .add_systems(PreUpdate, client_sync_update_handler)
            .add_systems(
                Update,
                (
                    player_kb_input_mapper,
                    (client_sync_start_handler, client_sync_update_handler, client_sync_stop_handler).chain(),
                    (handle_camera_input, update_billboard_transforms).chain(),
                ),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
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
