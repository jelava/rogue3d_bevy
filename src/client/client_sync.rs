use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use crate::{
    client::{
        components::Billboard,
        systems::{load_temp_asset_handles, TempAssetHandles},
    },
    common::{
        index::unique::{
            UniqueComponentIndex, UniqueComponentIndexPlugin, UniqueSparseComponentIndex,
        },
        ClientSyncStart, ClientSyncStop, ClientSyncUpdate, EntityKind, SharedId,
    },
};

pub struct LocalClientSyncPlugin;

impl Plugin for LocalClientSyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ClientSharedIdIndexPlugin::default())
            .add_systems(PreStartup, load_temp_asset_handles)
            .add_systems(
                PreUpdate,
                (
                    client_sync_start_handler,
                    client_sync_update_handler,
                    client_sync_stop_handler,
                )
                    .chain(),
            )
            .add_observer(update_unsynced_material_color)
            .add_observer(update_resynced_material_color);
    }
}

#[derive(Component, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ClientSharedId(SharedId);

impl From<SharedId> for ClientSharedId {
    fn from(value: SharedId) -> Self {
        Self(value)
    }
}

pub type ClientSharedIdIndex = UniqueSparseComponentIndex<ClientSharedId>;

pub type ClientSharedIdIndexPlugin =
    UniqueComponentIndexPlugin<ClientSharedId, ClientSharedIdIndex>;

#[derive(Component)]
pub struct Unsynced;

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
            // the entity is no longer unsynced
            commands.entity(entity).remove::<Unsynced>();

            let mut transform = transform_query.get_mut(entity).unwrap();
            transform.translation = pos_vec;
        } else {
            // no entity with a matching shared ID exists in the client, so spawn it

            match start_event.entity_kind {
                EntityKind::Player => commands.spawn((
                    EntityKind::Player,
                    ClientSharedId(start_event.shared_id),
                    Billboard,
                    Mesh3d(temp_asset_handles.rect_mesh_handle.clone()),
                    MeshMaterial3d(temp_asset_handles.player_material_handle.clone()),
                    Transform::from_translation(pos_vec),
                )),
                EntityKind::Npc => commands.spawn((
                    EntityKind::Npc,
                    ClientSharedId(start_event.shared_id),
                    Billboard,
                    Mesh3d(temp_asset_handles.rect_mesh_handle.clone()),
                    MeshMaterial3d(temp_asset_handles.npc_material_handle.clone()),
                    Transform::from_translation(pos_vec),
                )),
                EntityKind::Block => commands.spawn((
                    EntityKind::Block,
                    ClientSharedId(start_event.shared_id),
                    Mesh3d(temp_asset_handles.block_mesh_handle.clone()),
                    MeshMaterial3d(temp_asset_handles.block_material_handle.clone()),
                    Transform::from_translation(pos_vec),
                )),
                EntityKind::Brazier => commands.spawn((
                    EntityKind::Brazier,
                    ClientSharedId(start_event.shared_id),
                    Billboard,
                    Mesh3d(temp_asset_handles.rect_mesh_handle.clone()),
                    MeshMaterial3d(temp_asset_handles.brazier_material_handle.clone()),
                    NotShadowCaster,
                    NotShadowReceiver,
                    // todo: the light should probably be a separate child entity with a slightly offset position from the mesh
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

pub fn client_sync_stop_handler(
    mut commands: Commands,
    shared_id_index: Res<ClientSharedIdIndex>,
    mut stop_events: EventReader<ClientSyncStop>,
) {
    for stop_event in stop_events.read() {
        info!("Client received ClientSyncStop");
        // todo! despawn here? what about cases where entity is not necessarily fully despawned on
        // server but just temporarily not sending updates to client (i.e. entity that is no longer
        // seen by player)

        if let Some(&entity) = shared_id_index.get(&ClientSharedId(stop_event.shared_id)) {
            commands.entity(entity).insert(Unsynced);
        } else {
            panic!(
                "Could not find client entity with SharedId {:?}",
                stop_event.shared_id
            )
        }
    }
}

pub fn update_unsynced_material_color(
    trigger: Trigger<OnAdd, Unsynced>,
    temp_asset_handles: Res<TempAssetHandles>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut material_handle_query: Query<
        (&mut MeshMaterial3d<StandardMaterial>, &EntityKind),
        (With<ClientSharedId>, With<Unsynced>),
    >,
) {
    info!("mat update (unsynced)");

    if let Ok((mut handle, &entity_kind)) = material_handle_query.get_mut(trigger.target()) {
        *handle = match entity_kind {
            EntityKind::Block => {
                MeshMaterial3d(temp_asset_handles.unsynced_block_material_handle.clone())
            }
            EntityKind::Player => {
                MeshMaterial3d(temp_asset_handles.unsynced_player_material_handle.clone())
            }
            EntityKind::Npc => {
                MeshMaterial3d(temp_asset_handles.unsynced_npc_material_handle.clone())
            }
            EntityKind::Brazier => {
                MeshMaterial3d(temp_asset_handles.unsynced_brazier_material_handle.clone())
            }
        };
    } else {
        warn!("No material handle for entity?");
    }
}

pub fn update_resynced_material_color(
    trigger: Trigger<OnRemove, Unsynced>,
    temp_asset_handles: Res<TempAssetHandles>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut material_handle_query: Query<
        (&mut MeshMaterial3d<StandardMaterial>, &EntityKind),
        (With<ClientSharedId>, With<Unsynced>),
    >,
) {
    info!("mat update (resynced)");

    if let Ok((mut handle, &entity_kind)) = material_handle_query.get_mut(trigger.target()) {
        *handle = match entity_kind {
            EntityKind::Block => MeshMaterial3d(temp_asset_handles.block_material_handle.clone()),
            EntityKind::Player => MeshMaterial3d(temp_asset_handles.player_material_handle.clone()),
            EntityKind::Npc => MeshMaterial3d(temp_asset_handles.npc_material_handle.clone()),
            EntityKind::Brazier => {
                MeshMaterial3d(temp_asset_handles.brazier_material_handle.clone())
            }
        };
    } else {
        warn!("No material handle for entity?");
    }
}
