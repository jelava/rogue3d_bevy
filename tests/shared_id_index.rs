mod common;

use common::*;

use bevy::prelude::*;
use rogue3d_bevy::common::{
    index::unique::UniqueComponentIndex, SharedId, SharedIdIndex, SharedIdIndexPlugin,
};

#[test]
fn shared_id_index_test() {
    App::new()
        .add_plugins((BaseTestPlugins, SharedIdIndexPlugin::default()))
        .insert_resource(TestData {
            fixed_id: SharedId::new(),
        })
        .add_systems(
            Startup,
            (init_ids, check_init, modify_ids, check_modify, end_test).chain(),
        )
        .run();
}

#[derive(Resource)]
struct TestData {
    fixed_id: SharedId,
}

fn init_ids(mut commands: Commands) {
    commands.spawn(SharedId::new());
    commands.spawn(SharedId::new());
    commands.spawn(SharedId::new());
}

fn check_init(index: Res<SharedIdIndex>, query: Query<(Entity, &SharedId)>) {
    let mut count = 0;

    for (entity, id) in &query {
        assert_eq!(entity, *index.get(id).unwrap());
        count += 1;
    }

    assert_eq!(count, 3);
}

fn modify_ids(
    mut commands: Commands,
    index: Res<SharedIdIndex>,
    test_data: Res<TestData>,
    query: Query<(Entity, &SharedId)>,
) {
    let mut iter = query.iter();
    let (entity, old_id) = iter.next().unwrap();

    // Change the ID of the 1st entity
    commands.entity(entity).insert(test_data.fixed_id);

    // Despawn the next entity
    commands.entity(iter.next().unwrap().0).despawn();

    // Note that the component hooks haven't run yet so the index does not reflect the new ID for the modified entity
    assert_eq!(index.get(old_id), Some(&entity));
    assert!(index.get(&test_data.fixed_id).is_none());
}

fn check_modify(
    index: Res<SharedIdIndex>,
    test_data: Res<TestData>,
    query: Query<(Entity, &SharedId)>,
) {
    assert!(index.get(&test_data.fixed_id).is_some());

    let mut count = 0;

    for (entity, id) in &query {
        assert_eq!(entity, *index.get(id).unwrap());
        count += 1;
    }

    // This time the count should only be 2 (one entity was despawned and the newly spawned one
    // should not be in the index due to duplicate SharedId)
    assert_eq!(count, 2);
}
