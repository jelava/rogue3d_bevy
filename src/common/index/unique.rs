use std::{hash::Hash, marker::PhantomData};

use bevy::{ecs::component::HookContext, platform::collections::HashMap, prelude::*};

pub trait UniqueComponentIndex<C: Component>: Resource {
    fn get(&self, component: &C) -> Option<&Entity>;
    fn try_insert(&mut self, component: C, entity: Entity) -> Result<(), NonUniqueComponentError>;
    fn remove(&mut self, component: &C);
}

#[derive(Debug)]
pub struct NonUniqueComponentError;

#[derive(Resource)]
pub struct UniqueSparseComponentIndex<C: Component + Eq + Hash> {
    data: HashMap<C, Entity>,
}

impl<C: Component + Eq + Hash> Default for UniqueSparseComponentIndex<C> {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl<C: Component + Eq + Hash> UniqueComponentIndex<C> for UniqueSparseComponentIndex<C> {
    fn get(&self, component: &C) -> Option<&Entity> {
        self.data.get(component)
    }

    fn try_insert(&mut self, component: C, entity: Entity) -> Result<(), NonUniqueComponentError> {
        if !self.data.contains_key(&component) {
            self.data.insert(component, entity);
            Ok(())
        } else {
            Err(NonUniqueComponentError)
        }
    }

    fn remove(&mut self, component: &C) {
        self.data.remove(component);
    }
}

pub struct UniqueComponentIndexPlugin<C: Component + Copy + Clone, I: UniqueComponentIndex<C>>(
    PhantomData<(C, I)>,
);

impl<C: Component + Copy + Clone, I: UniqueComponentIndex<C>> Default
    for UniqueComponentIndexPlugin<C, I>
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C: Component + Copy + Clone, I: UniqueComponentIndex<C> + Default> Plugin
    for UniqueComponentIndexPlugin<C, I>
{
    fn build(&self, app: &mut App) {
        app.init_resource::<I>()
            .add_systems(Startup, register_unique_component_index_hooks::<C, I>);
    }
}

fn register_unique_component_index_hooks<
    C: Component + Copy + Clone,
    I: UniqueComponentIndex<C>,
>(
    world: &mut World,
) {
    world
        .register_component_hooks::<C>()
        .on_insert(|mut world, HookContext { entity, .. }| {
            let component = *world.get::<C>(entity).unwrap();
            let result = world.resource_mut::<I>().try_insert(component, entity);

            // todo: use the result in a better way
            if result.is_err() {
                warn!("Tried to insert duplicate component into a unique component index");
            }
        })
        .on_replace(|mut world, HookContext { entity, .. }| {
            let component = *world.get(entity).unwrap();
            world.resource_mut::<I>().remove(&component);

            // The insert hook is guaranteed to run after this if the component is being replaced and
            // it will handle re-adding the entity to the index for the new component value
        });
}
