use bevy::{log::LogPlugin, prelude::*};

use rogue3d_bevy::common::{
    grid::{GridPosition, SparseGridIndex},
    index::{ComponentIndex, ComponentIndexPlugin},
};

// Generic set of basic tests for anything that implements GridIndex (specific tests below)

fn basic_grid_index_tests<I: ComponentIndex<GridPosition> + Default>() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .add_plugins(ComponentIndexPlugin::<GridPosition, I>::default())
        .add_systems(Startup, init_spawns)
        .add_systems(PostStartup, check_init::<I>)
        .add_systems(Update, tick::<I>)
        .add_systems(PostUpdate, check_tick::<I>)
        .run();
}

fn init_spawns(mut commands: Commands) {
    info!("spawning ents w/ grid position");
    commands.spawn(GridPosition(IVec3::new(1, 2, 3)));
    commands.spawn(GridPosition(IVec3::new(0, 0, 0)));

    info!("init done");
}

fn check_init<I: ComponentIndex<GridPosition>>(grid_index: Res<I>, pos_query: Query<&GridPosition>) {
    for pos in &pos_query {
        info!("checking pos {:?}", pos);
        let pos_entities = grid_index.get(pos).expect("Expected to find entities");
        assert_eq!(pos_entities.len(), 1);
        assert!(pos_query.get(*pos_entities.iter().next().unwrap()).is_ok())
    }

    info!("grid index successfully updated after init");
}

fn tick<I: ComponentIndex<GridPosition>>(mut commands: Commands, grid_index: Res<I>) {
    // move the position of the entity at (1, 2, 3) to test grid index update
    let entities_at_123 = grid_index
        .get(&GridPosition(IVec3::new(1, 2, 3)))
        .expect("Expected to find entity at (1, 2, 3)");

    assert_eq!(entities_at_123.len(), 1);
    
    let entity_at_123 = entities_at_123.iter().next().unwrap();

    commands
        .entity(*entity_at_123)
        .insert(GridPosition(IVec3::new(2, 3, 4)));

    // despawn the entity at (0, 0, 0) to test index removal
    let entities_at_000 = grid_index
        .get(&GridPosition(IVec3::new(0, 0, 0)))
        .expect("Expected to find entity at (0, 0, 0)");

    assert_eq!(entities_at_000.len(), 1);

    let entity_at_000 = *entities_at_000.iter().next().unwrap();

    commands.entity(entity_at_000).despawn();
}

fn check_tick<I: ComponentIndex<GridPosition>>(
    grid_index: Res<I>,
    mut app_exit: EventWriter<AppExit>,
    pos_query: Query<Entity, With<GridPosition>>,
) {
    // there should be only one entity with a position at (2, 3, 4)
    let pos_entity = pos_query.single().expect("Query should get one entity");

    // this should just be a different way of getting that same entity
    let entities_at_234 = grid_index
        .get(&GridPosition(IVec3::new(2, 3, 4)))
        .expect("Expected entity at (2, 3, 4)");

    assert_eq!(entities_at_234.len(), 1);

    let entity_at_234 = *entities_at_234.iter().next().unwrap();

    assert_eq!(pos_entity, entity_at_234);

    // nothing should be indexed at (0, 0, 0) since that entity was despawned
    assert!(grid_index.get(&GridPosition(IVec3::ZERO)).is_none());

    // nothing should be indexed at (1, 2, 3) since that entity moved
    assert!(grid_index.get(&GridPosition(IVec3::new(1, 2, 3))).is_none());

    info!("grid index successfully updated after update, exiting...");
    app_exit.write(AppExit::Success);
}

// Basic test for specific index types

#[test]
fn basic_sparse_grid_index_tests() {
    basic_grid_index_tests::<SparseGridIndex>();
}

/*
#[test]
fn basic_grid_chunk_index_tests() {
    basic_grid_index_tests::<GridChunkIndex<5>>();
}
*/
