use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct GridPosition(pub IVec3);

#[derive(Component)]
pub enum GridShape {
    SingleBlock,
}

/// Stores data in a hash map. Good for space efficiency if data is not expected to be densely packed.
pub struct SparseGridData<T> {
    data: HashMap<IVec3, T>,
}
