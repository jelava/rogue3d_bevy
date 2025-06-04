mod common;

use common::*;

use bevy::prelude::*;
use rogue3d_bevy::{
    client::client_sync::{
        client_sync_stop_handler, client_sync_update_handler, ClientSharedId, ClientSharedIdIndex,
        ClientSharedIdIndexPlugin, LocalClientSyncPlugin, Unsynced,
    },
    common::{
        grid::GridPosition, index::unique::UniqueComponentIndex, ClientSyncStart, EntityKind,
        LocalBridgePlugin,
    },
    server::server_sync::{ClientSync, LocalServerSyncPlugin, ServerSharedId},
};

struct TestClientSyncPlugin;

// This is basically LocalClientSyncPlugin without the asset/rendering stuff
impl Plugin for TestClientSyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ClientSharedIdIndexPlugin::default())
            .add_systems(
                PreUpdate,
                (
                    test_sync_start_handler,
                    client_sync_update_handler,
                    client_sync_stop_handler,
                )
                    .chain(),
            );
    }
}

fn test_sync_start_handler(
    mut commands: Commands,
    shared_id_index: Res<ClientSharedIdIndex>,
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

        if let Some(&entity) = shared_id_index.get(&start_event.shared_id.into()) {
            // the entity is no longer unsynced
            commands.entity(entity).remove::<Unsynced>();

            let mut transform = transform_query.get_mut(entity).unwrap();
            transform.translation = pos_vec;
        } else {
            // no entity with a matching shared ID exists in the client, so spawn it
            let id: ClientSharedId = start_event.shared_id.into();

            match start_event.entity_kind {
                EntityKind::Player => commands.spawn((
                    EntityKind::Player,
                    // ClientSharedId(start_event.shared_id),
                    id,
                    Transform::from_translation(pos_vec),
                )),
                EntityKind::Npc => commands.spawn((
                    EntityKind::Npc,
                    // ClientSharedId(start_event.shared_id),
                    id,
                    Transform::from_translation(pos_vec),
                )),
                EntityKind::Block => commands.spawn((
                    EntityKind::Block,
                    // ClientSharedId(start_event.shared_id),
                    id,
                    Transform::from_translation(pos_vec),
                )),
                EntityKind::Brazier => commands.spawn((
                    EntityKind::Brazier,
                    // ClientSharedId(start_event.shared_id),
                    id,
                    Transform::from_translation(pos_vec),
                )),
            };
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum SyncMovementTestState {
    #[default]
    Init,
    Move,
    CheckMove,
}

#[test]
fn sync_movement_test() {
    App::new()
        .add_plugins((
            BaseTestPlugins,
            LocalBridgePlugin,
            TestClientSyncPlugin,
            LocalServerSyncPlugin,
        ))
        .init_state::<SyncMovementTestState>()
        .add_systems(Startup, init)
        .add_systems(
            Update,
            (
                check_init.run_if(in_state(SyncMovementTestState::Init)),
                move_entity.run_if(in_state(SyncMovementTestState::Move)),
                (check_move, end_test)
                    .chain()
                    .run_if(in_state(SyncMovementTestState::CheckMove)),
            ),
        )
        .run();

    fn init(mut commands: Commands) {
        // EntityKind required here for now due to lazy/simplistic implementation, will change later
        commands.spawn((
            ServerSharedId::new(),
            ClientSync,
            GridPosition(IVec3::ZERO),
            EntityKind::Npc,
        ));
    }

    fn check_init(
        client_query: Query<(&ClientSharedId, &Transform)>,
        // server_query: Query<&ServerSharedId>,
        mut next_state: ResMut<NextState<SyncMovementTestState>>,
    ) {
        let (client_id, transform) = client_query
            .single()
            .expect("Should have spawned client entity");

        // let server_id = server_query.single().unwrap();

        // assert_eq!(client_id.0, server_id.0);
        assert_eq!(transform.translation, Vec3::ZERO);

        next_state.set(SyncMovementTestState::Move);
    }

    fn move_entity(
        mut commands: Commands,
        server_entity_query: Query<Entity, With<ServerSharedId>>,
        mut next_state: ResMut<NextState<SyncMovementTestState>>,
    ) {
        let entity = server_entity_query.single().unwrap();

        commands
            .entity(entity)
            .insert(GridPosition(IVec3::new(1, 1, 1)));

        next_state.set(SyncMovementTestState::CheckMove);
    }

    fn check_move(client_query: Query<&Transform, With<ClientSharedId>>) {
        let transform = client_query.single().unwrap();

        assert_eq!(transform.translation, Vec3::new(1.0, 1.0, 1.0));
    }
}

#[test]
fn add_remove_sync_test() {
    todo!();
}
