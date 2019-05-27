//! Implements storage for Component instances, in a way that allows us to 
//! short-circuit the rendering process so we don't have to re-scan entire 
//! tree structures when updating state.

use std::collections::HashMap;

pub use alchemy_styles::Appearance;
use alchemy_styles::stretch::node::{Node as LayoutNode};

use crate::reconciler::error::{RenderEngineError as Error};
use crate::reconciler::key::{Allocator, Id, INSTANCE_ALLOCATOR, ComponentKey};
use crate::traits::Component;

/// This is a clone of a structure you'll also find over in stretch. We do this separately 
/// here for two reasons.
///
/// - First, a Component may have children that don't require styles or layout passes. These nodes 
/// should not have `Style` or `Appearance` nodes created, but we do need the correct parent/child 
/// relationships in place.
/// - The `Storage` pieces of stretch are realistically an implementation detail that we shouldn't 
/// rely on. 
struct Storage<T>(HashMap<ComponentKey, T>);

impl<T> Storage<T> {
    pub fn new() -> Self {
        Storage(HashMap::new())
    }

    pub fn get(&self, key: ComponentKey) -> Result<&T, Error> {
        match self.0.get(&key) {
            Some(v) => Ok(v),
            None => Err(Error::InvalidComponentKey(key)),
        }
    }

    pub fn get_mut(&mut self, key: ComponentKey) -> Result<&mut T, Error> {
        match self.0.get_mut(&key) {
            Some(v) => Ok(v),
            None => Err(Error::InvalidComponentKey(key)),
        }
    }

    pub fn insert(&mut self, key: ComponentKey, value: T) -> Option<T> {
        self.0.insert(key, value)
    }
}

impl<T> std::ops::Index<&ComponentKey> for Storage<T> {
    type Output = T;

    fn index(&self, idx: &ComponentKey) -> &T {
        &(self.0)[idx]
    }
}

pub struct Instance {
    component: Box<Component>,
    appearance: Appearance,
    layout: Option<LayoutNode>
}

pub(crate) struct ComponentStore {
    id: Id,
    nodes: Allocator,
    components: Storage<Instance>,
    parents: Storage<Vec<ComponentKey>>,
    children: Storage<Vec<ComponentKey>>
}

impl ComponentStore {
    pub fn new() -> Self {
        ComponentStore {
            id: INSTANCE_ALLOCATOR.lock().unwrap().allocate(),
            nodes: Allocator::new(),
            components: Storage::new(),
            parents: Storage::new(),
            children: Storage::new()
        }
    }

    fn allocate_node(&mut self) -> ComponentKey {
        let local = self.nodes.allocate();
        ComponentKey { instance: self.id, local }
    }

    pub fn new_node<C: Component + 'static>(&mut self, component: C, layout_key: Option<LayoutNode>, children: Vec<ComponentKey>) -> Result<ComponentKey, Error> {
        let key = self.allocate_node();

        for child in &children {
            self.parents.get_mut(*child)?.push(key);
        }

        self.components.insert(key, Instance {
            component: Box::new(component),
            appearance: Appearance::default(),
            layout: layout_key
        });

        self.parents.insert(key, Vec::with_capacity(1));
        self.children.insert(key, children);

        Ok(key)
    }

    pub fn add_child(&mut self, key: ComponentKey, child: ComponentKey) -> Result<(), Error> {
        self.parents.get_mut(child)?.push(key);
        self.children.get_mut(key)?.push(child);
        Ok(())
    }

    pub fn set_children(&mut self, key: ComponentKey, children: Vec<ComponentKey>) -> Result<(), Error> {
        // Remove node as parent from all its current children.
        for child in self.children.get(key)? {
            self.parents.get_mut(*child)?.retain(|p| *p != key);
        }

        *self.children.get_mut(key)? = Vec::with_capacity(children.len());

        // Build up relation node <-> child
        for child in children {
            self.parents.get_mut(child)?.push(key);
            self.children.get_mut(key)?.push(child);
        }

        Ok(())
    }

    pub fn remove_child(&mut self, key: ComponentKey, child: ComponentKey) -> Result<ComponentKey, Error> {
        match self.children(key)?.iter().position(|n| *n == child) {
            Some(index) => self.remove_child_at_index(key, index),
            None => Err(Error::InvalidComponentKey(child)),
        }
    }

    pub fn remove_child_at_index(&mut self, key: ComponentKey, index: usize) -> Result<ComponentKey, Error> {
        let child = self.children.get_mut(key)?.remove(index);
        self.parents.get_mut(child)?.retain(|p| *p != key);
        Ok(child)
    }

    pub fn replace_child_at_index(&mut self, key: ComponentKey, index: usize, child: ComponentKey) -> Result<ComponentKey, Error> {
        self.parents.get_mut(child)?.push(key);
        let old_child = std::mem::replace(&mut self.children.get_mut(key)?[index], child);
        self.parents.get_mut(old_child)?.retain(|p| *p != key);
        Ok(old_child)
    }

    pub fn children(&self, key: ComponentKey) -> Result<Vec<ComponentKey>, Error> {
        self.children.get(key).map(Clone::clone)
    }

    pub fn child_count(&self, key: ComponentKey) -> Result<usize, Error> {
        self.children.get(key).map(Vec::len)
    }

    pub fn get(&self, key: ComponentKey) -> Result<&Instance, Error> {
        self.components.get(key)
    }
    
    pub fn get_mut(&mut self, key: ComponentKey) -> Result<&mut Instance, Error> {
        self.components.get_mut(key)
    }
}
