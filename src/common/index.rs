use std::hash::Hash;

use bevy::{
    ecs::{component::HookContext, entity::EntityHashSet},
    platform::collections::HashMap,
    prelude::*,
};

/*
pub trait UniqueComponentIndex<C: Component>: Resource {
    fn get(&self, component: &C) -> Option<Entity>;
    fn try_insert(&mut self, component: &C, entity: Entity) -> bool;
    fn remove(&mut self, component: &C);
}
*/

/// Doesn't require adding any actual functions (yet?), just a shorthand to avoid having to write
/// Component + Copy + Clone repeatedly
// pub trait IndexableComponent: Component + Copy + Clone {}

// impl<C: Component + Copy + Clone> IndexableComponent for C {}

pub trait ComponentIndex<C: Component>: Resource {
    // type Cmp: Component + Copy + Clone;

    fn get(&self, component: &C) -> Option<&EntityHashSet>;
    fn insert(&mut self, component: C, entity: Entity);
    fn remove(&mut self, component: &C, entity: Entity);
}

/// Useful for keeping track of locations of things that are scattered across a wide area with no extra
/// memory overhead, or which don't need to be efficiently accessible in sequence (an underlying data
/// structure with better spatial locality will do better for that).
#[derive(Resource)]
pub struct SparseComponentIndex<C: Component + Copy + Clone + Hash + Eq> {
    data: HashMap<C, EntityHashSet>,
}

// Manually implement rather than derive to prevent complaints about T not implementing Default
impl<C: Component + Copy + Clone + Hash + Eq> Default for SparseComponentIndex<C> {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl<C: Component + Copy + Clone + Hash + Eq> ComponentIndex<C> for SparseComponentIndex<C> {
    // type Cmp = C;

    fn get(&self, component: &C) -> Option<&EntityHashSet> {
        self.data.get(component)
    }

    fn insert(&mut self, component: C, entity: Entity) {
        if let Some(entity_set) = self.data.get_mut(&component) {
            entity_set.insert(entity);
        } else {
            self.data.insert(component, EntityHashSet::from([entity]));
        }
    }

    fn remove(&mut self, component: &C, entity: Entity) {
        if let Some(entity_set) = self.data.get_mut(component) {
            entity_set.remove(&entity);

            if entity_set.is_empty() {
                self.data.remove(component);
            }
        }
    }
}

// #[derive(Default)]
pub struct ComponentIndexPlugin<C: Component + Copy + Clone, I: ComponentIndex<C> + Default>(std::marker::PhantomData<(C, I)>);

impl<C: Component + Copy + Clone, I: ComponentIndex<C> + Default> Default for ComponentIndexPlugin<C, I> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<C: Component + Copy + Clone, I: ComponentIndex<C> + Default> Plugin for ComponentIndexPlugin<C, I> {
    fn build(&self, app: &mut App) {
        app.init_resource::<I>()
            .add_systems(Startup, init_component_index_hooks::<C, I>);
    }
}

fn init_component_index_hooks<C: Component + Copy + Clone, I: ComponentIndex<C>>(world: &mut World) {
    world
        .register_component_hooks::<C>()
        .on_insert(|mut world, HookContext { entity, .. }| {
            let component = *world.get(entity).unwrap();
            world.resource_mut::<I>().insert(component, entity);
        })
        .on_replace(|mut world, HookContext { entity, .. }| {
            let component = *world.get(entity).unwrap();
            world.resource_mut::<I>().remove(&component, entity);

            // The insert hook is guaranteed to run after this if the component is being replaced and
            // it will handle re-adding the entity to the index for the new component value
        });
}
