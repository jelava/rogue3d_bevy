use bevy::{log::LogPlugin, prelude::*};

use rogue3d_bevy::common::grid::{
    // GridChunkIndex,
    GridIndex,
    GridPosition,
    SparseGridIndex,
};

// Generic set of basic tests for anything that implements GridIndex (specific tests below)

fn basic_grid_index_tests<I: GridIndex<Entity> + Resource + Default>() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .init_resource::<I>()
        .add_systems(Startup, (init_grid_index_hooks::<I>, init_spawns).chain())
        .add_systems(PostStartup, check_init::<I>)
        .add_systems(Update, tick::<I>)
        .add_systems(PostUpdate, check_tick::<I>)
        .run();
}

fn init_grid_index_hooks<I: GridIndex<Entity> + Resource>(world: &mut World) {
    info!("setting up component hooks");

    world
        .register_component_hooks::<GridPosition>()
        .on_insert(|mut world, entity, _component_id| {
            let &GridPosition(pos) = world.get(entity).unwrap();
            let mut grid_index = world.resource_mut::<I>();

            if !grid_index.try_insert(pos, entity) {
                panic!("Failed to insert entity into grid index at position {:?} (probably already occupied)", pos);
            }
        })
        .on_replace(|mut world, entity, _component_id| {
            let &GridPosition(pos) = world.get(entity).unwrap();
            let mut grid_index = world.resource_mut::<I>();
            grid_index.remove(pos);

            // The insert hook is guaranteed to run after this if the component is being replaced and
            // it will handle adding the entity at the new pos.
        });
}

fn init_spawns(mut commands: Commands) {
    info!("spawning ents w/ grid position");
    commands.spawn(GridPosition(IVec3::new(1, 2, 3)));
    commands.spawn(GridPosition(IVec3::new(0, 0, 0)));

    info!("init done");
}

fn check_init<I: GridIndex<Entity> + Resource>(
    grid_index: Res<I>,
    pos_query: Query<&GridPosition>,
) {
    for GridPosition(pos) in &pos_query {
        info!("checking pos {:?}", pos);
        let pos_entity = grid_index.get(*pos).expect("Expected to find entity");
        assert!(pos_query.get(*pos_entity).is_ok());
    }

    info!("grid index successfully updated after init");
}

fn tick<I: GridIndex<Entity> + Resource>(mut commands: Commands, grid_index: Res<I>) {
    // move the position of the entity at (1, 2, 3) to test grid index update
    let entity_at_123 = grid_index
        .get(IVec3::new(1, 2, 3))
        .expect("Expected to find entity at (1, 2, 3)");

    commands
        .entity(*entity_at_123)
        .insert(GridPosition(IVec3::new(2, 3, 4)));

    // despawn the entity at (0, 0, 0) to test index removal
    let entity_at_000 = grid_index
        .get(IVec3::new(0, 0, 0))
        .expect("Expected to find entity at (0, 0, 0)");

    commands.entity(*entity_at_000).despawn();
}

fn check_tick<I: GridIndex<Entity> + Resource>(
    grid_index: Res<I>,
    mut app_exit: EventWriter<AppExit>,
    pos_query: Query<Entity, With<GridPosition>>,
) {
    // there should be only one entity with a position at (2, 3, 4)
    let pos_entity = pos_query.single();

    // this should just be a different way of getting that same entity
    let entity_at_234 = grid_index
        .get(IVec3::new(2, 3, 4))
        .expect("Expected entity at (2, 3, 4)");

    assert_eq!(pos_entity, *entity_at_234);

    // nothing should be indexed at (0, 0, 0) since that entity was despawned
    assert!(grid_index.get(IVec3::ZERO).is_none());

    // nothing should be indexed at (1, 2, 3) since that entity moved
    assert!(grid_index.get(IVec3::new(1, 2, 3)).is_none());

    info!("grid index successfully updated after update, exiting...");
    app_exit.send(AppExit::Success);
}

// Basic test for specific index types

#[test]
fn basic_sparse_grid_index_tests() {
    basic_grid_index_tests::<SparseGridIndex<Entity>>();
}
/*
#[test]
fn basic_grid_chunk_index_tests() {
    basic_grid_index_tests::<GridChunkIndex<5>>();
}
*/
