mod components;
mod input;
mod systems;

use bevy::{
    app::{Plugin, Startup, Update},
    core_pipeline::core_3d::Camera3d,
    prelude::{default, Commands, DefaultPlugins, IntoScheduleConfigs, PluginGroup},
    window::{Window, WindowPlugin},
};

use crate::client::{
    input::{systems::*, PlayerInputMap},
    systems::*,
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        use std::default::Default;

        let default_plugins = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "rogue3d".into(),
                ..default()
            }),
            ..default()
        });

        app.add_plugins(default_plugins)
            .insert_resource(PlayerInputMap::default())
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    player_kb_input_mapper,
                    (handle_spawns, handle_position_updates).chain(),
                    (handle_camera_input, update_billboard_transforms).chain(),
                ),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}
