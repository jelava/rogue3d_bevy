use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
    utils::HashMap,
};

#[derive(Component)]
// #[component(
// on_add = on_add_grid_position_hook,
// on_insert = on_insert_grid_position_hook,
// on_remove = on_remove_grid_position_hook,
// on_replace = on_replace_grid_position_hook
// )]
pub struct GridPosition(pub IVec3);

#[derive(Component)]
pub enum GridShape {
    SingleBlock,
}

/// One to (at most) one mapping of position on grid (IVec3) to instance of type T. Primarily intended
/// for data structures that will implement it for Bevy entities as T, but keeping the trait generic
/// in case it turns out to be useful for other types too. Currently the interface is very minimal,
/// basically just what is required for component hooks to maintain an index of entities by position.
pub trait GridIndex<T> {
    fn get(&self, pos: IVec3) -> Option<&T>;
    fn try_insert(&mut self, pos: IVec3, data: T) -> bool;
    fn remove(&mut self, pos: IVec3);
}

/// Useful for keeping track of locations of things that are scattered across a wide area with no extra
/// memory overhead, or which don't need to be efficiently accessible in sequence (an underlying data
/// structure with better spatial locality will do better for that).
#[derive(Resource)]
pub struct SparseGridIndex<T> {
    data: HashMap<IVec3, T>,
}

// Manually implement rather than derive to prevent complaints about T not implementing Default
impl<T> Default for SparseGridIndex<T> {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl<T> GridIndex<T> for SparseGridIndex<T> {
    fn get(&self, pos: IVec3) -> Option<&T> {
        self.data.get(&pos)
    }

    fn try_insert(&mut self, pos: IVec3, data: T) -> bool {
        self.data.try_insert(pos, data).is_ok()
    }

    fn remove(&mut self, pos: IVec3) {
        self.data.remove(&pos);
    }
}

// Array-based index for tracking locations in a fixed N*N*N cubic chunk of the grid. Best suited
// for densely packed data that is clustered in a small area.
// TODO: correctly initializing the data is kinda clunky to work with and prone to stack overflows. Needs reworking or removal if not needed.
/*
#[derive(Resource)]
pub struct GridChunkIndex<T, const N: usize> {
    origin: IVec3,
    data: Box<[[[Option<T>; N]; N]; N]>,
}

impl<const N: usize> Default for GridChunkIndex<N> {
    fn default() -> Self {
        Self::new(IVec3::ZERO)
    }
}


impl<T, const N: usize> GridChunkIndex<T, N> {
    // TODO: Right now sufficiently large N will cause stack overflows during initialization
    fn new(origin: IVec3) -> Self {
        // This is kinda clunky but helps avoid (but doesn't entirely prevent) stack overflows from
        // trying the initialize the arrays directly
        let data: Box<[[[Option<T>; N]; N]; N]> = vec![None; N * N * N] // completely flat (N * N * N)
            .chunks_exact(N)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<_>>() // Nested once (N * N arrays of N)
            .chunks_exact(N)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        // let data2: Box<[[[Option<T>; N]; N]; N]> = vec![vec![vec![None; N]; N]; N]


        Self { origin, data }
    }
}

impl<T, const N: usize> GridIndex<T> for GridChunkIndex<T, N> {
    fn get(&self, pos: IVec3) -> Option<&T> {
        let index = pos - self.origin;

        if index.min_element() >= 0 && (index.max_element() as usize) < N {
            self.data[index.x as usize][index.y as usize][index.z as usize].as_ref()
        } else {
            None
        }
    }

    fn try_insert(&mut self, pos: IVec3, data: T) -> bool {
        let index = pos - self.origin;

        if index.min_element() >= 0 && (index.max_element() as usize) < N {
            if self.data[index.x as usize][index.y as usize][index.z as usize].is_none() {
                self.data[index.x as usize][index.y as usize][index.z as usize] = Some(data);
                return true;
            }
        }

        false
    }

    fn remove(&mut self, pos: IVec3) {
        let index = pos - self.origin;

        if index.min_element() >= 0 && (index.max_element() as usize) < N {
            self.data[index.x as usize][index.y as usize][index.z as usize] = None;
        }
    }
}
*/
