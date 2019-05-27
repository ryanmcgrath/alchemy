//! Implements an auto-incrementing ID for Component instances.

use std::sync::Mutex;

use alchemy_styles::lazy_static;

lazy_static! {
    /// Global stretch instance id allocator.
    pub(crate) static ref INSTANCE_ALLOCATOR: Mutex<Allocator> = Mutex::new(Allocator::new());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Id {
    id: u32
}

pub(crate) struct Allocator {
    new_id: u32
}

impl Allocator {
    pub fn new() -> Self {
        Allocator { new_id: 1 }
    }

    pub fn allocate(&mut self) -> Id {
        let id = self.new_id;
        self.new_id += 1;
        Id { id: id }
    }
}

/// Used as a key for Component storage. Component instances receive these
/// in their constructor methods, and should retain them as a tool to update their 
/// state.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentKey {
    pub(crate) instance: Id,
    pub(crate) local: Id,
}

impl ComponentKey {
    /// A placeholder value, used purely for ensuring the diffing algorithm remains 
    /// readable by reducing some unwrapping hell.
    pub fn placeholder() -> ComponentKey {
        ComponentKey {
            instance: Id { id: 0 },
            local: Id { id: 0 }
        }
    }
}
