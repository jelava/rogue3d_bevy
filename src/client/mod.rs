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
        grid::SparseGridIndexPlugin,
        index::unique::{UniqueComponentIndexPlugin, UniqueSparseComponentIndex},
        SharedId, SharedIdIndexPlugin,
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
            .add_systems(Startup, spawn_camera)
            .add_systems(PreUpdate, handle_client_updates)
            .add_systems(
                Update,
                (
                    player_kb_input_mapper,
                    // (handle_spawns, handle_position_updates).chain(),
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

type ClientSharedIdIndexPlugin =
    UniqueComponentIndexPlugin<ClientSharedId, UniqueSparseComponentIndex<ClientSharedId>>;
