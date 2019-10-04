//! This module is included while awaiting an upstream merge in stretch proper.
//! You should not rely on it, and consider it an implementation detail.

use core::any::Any;

use std::collections::HashMap;
use std::ops::Drop;
use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::stretch::geometry::Size;
use crate::stretch::id;
use crate::stretch::number::Number;
use crate::stretch::result::{Cache, Layout};
use crate::stretch::style::*;
use crate::stretch::Error;

type MeasureFunc = Box<dyn Fn(Size<Number>) -> Result<Size<f32>, Box<dyn Any>> + Send + Sync + 'static>;

lazy_static! {
    /// Global stretch instance id allocator.
    static ref INSTANCE_ALLOCATOR: Mutex<id::Allocator> = Mutex::new(id::Allocator::new());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Node {
    instance: id::Id,
    local: id::Id,
}

pub(crate) struct Storage<T>(HashMap<Node, T>);

impl<T> Storage<T> {
    pub fn new() -> Self {
        Storage(HashMap::new())
    }

    pub fn get(&self, node: Node) -> Result<&T, Error> {
        match self.0.get(&node) {
            Some(v) => Ok(v),
            None => Err(Error::InvalidNode(node)),
        }
    }

    pub fn get_mut(&mut self, node: Node) -> Result<&mut T, Error> {
        match self.0.get_mut(&node) {
            Some(v) => Ok(v),
            None => Err(Error::InvalidNode(node)),
        }
    }

    pub fn insert(&mut self, node: Node, value: T) -> Option<T> {
        self.0.insert(node, value)
    }
}

impl<T> std::ops::Index<&Node> for Storage<T> {
    type Output = T;

    fn index(&self, idx: &Node) -> &T {
        &(self.0)[idx]
    }
}

pub struct Stretch {
    id: id::Id,
    nodes: id::Allocator,
    pub(crate) style: Storage<Style>,
    pub(crate) parents: Storage<Vec<Node>>,
    pub(crate) children: Storage<Vec<Node>>,
    pub(crate) measure: Storage<Option<MeasureFunc>>,
    pub(crate) layout: Storage<Layout>,
    pub(crate) layout_cache: Storage<Option<Cache>>,
    pub(crate) is_dirty: Storage<bool>,
}

impl Stretch {
    pub fn new() -> Self {
        Stretch {
            id: INSTANCE_ALLOCATOR.lock().unwrap().allocate(),
            nodes: id::Allocator::new(),
            style: Storage::new(),
            parents: Storage::new(),
            children: Storage::new(),
            measure: Storage::new(),
            layout: Storage::new(),
            layout_cache: Storage::new(),
            is_dirty: Storage::new(),
        }
    }

    fn allocate_node(&mut self) -> Node {
        let local = self.nodes.allocate();
        Node { instance: self.id, local }
    }

    pub fn new_leaf(&mut self, style: Style, measure: MeasureFunc) -> Node {
        let node = self.allocate_node();

        self.style.insert(node, style);
        self.parents.insert(node, Vec::with_capacity(1));
        self.children.insert(node, Vec::with_capacity(0));
        self.measure.insert(node, Some(measure));
        self.layout.insert(node, Layout::new());
        self.layout_cache.insert(node, None);
        self.is_dirty.insert(node, true);

        node
    }

    pub fn new_node(&mut self, style: Style, children: Vec<Node>) -> Result<Node, Error> {
        let node = self.allocate_node();

        for child in &children {
            self.parents.get_mut(*child)?.push(node);
        }

        self.style.insert(node, style);
        self.parents.insert(node, Vec::with_capacity(1));
        self.children.insert(node, children);
        self.measure.insert(node, None);
        self.layout.insert(node, Layout::new());
        self.layout_cache.insert(node, None);
        self.is_dirty.insert(node, true);

        Ok(node)
    }

    pub fn set_measure(&mut self, node: Node, measure: Option<MeasureFunc>) -> Result<(), Error> {
        *self.measure.get_mut(node)? = measure;
        self.mark_dirty(node)?;
        Ok(())
    }

    pub fn add_child(&mut self, node: Node, child: Node) -> Result<(), Error> {
        self.parents.get_mut(child)?.push(node);
        self.children.get_mut(node)?.push(child);
        self.mark_dirty(node)
    }

    pub fn set_children(&mut self, node: Node, children: Vec<Node>) -> Result<(), Error> {
        // Remove node as parent from all its current children.
        for child in self.children.get(node)? {
            self.parents.get_mut(*child)?.retain(|p| *p != node);
        }

        *self.children.get_mut(node)? = Vec::with_capacity(children.len());

        // Build up relation node <-> child
        for child in children {
            self.parents.get_mut(child)?.push(node);
            self.children.get_mut(node)?.push(child);
        }

        self.mark_dirty(node)
    }

    pub fn remove_child(&mut self, node: Node, child: Node) -> Result<Node, Error> {
        match self.children(node)?.iter().position(|n| *n == child) {
            Some(index) => self.remove_child_at_index(node, index),
            None => Err(Error::InvalidNode(child)),
        }
    }

    pub fn remove_child_at_index(&mut self, node: Node, index: usize) -> Result<Node, Error> {
        let child = self.children.get_mut(node)?.remove(index);
        self.parents.get_mut(child)?.retain(|p| *p != node);

        self.mark_dirty(node)?;

        Ok(child)
    }

    pub fn replace_child_at_index(&mut self, node: Node, index: usize, child: Node) -> Result<Node, Error> {
        self.parents.get_mut(child)?.push(node);
        let old_child = std::mem::replace(&mut self.children.get_mut(node)?[index], child);
        self.parents.get_mut(old_child)?.retain(|p| *p != node);

        self.mark_dirty(node)?;

        Ok(old_child)
    }

    pub fn children(&self, node: Node) -> Result<Vec<Node>, Error> {
        self.children.get(node).map(Clone::clone)
    }

    pub fn child_count(&self, node: Node) -> Result<usize, Error> {
        self.children.get(node).map(Vec::len)
    }

    pub fn set_style(&mut self, node: Node, style: Style) -> Result<(), Error> {
        *self.style.get_mut(node)? = style;
        self.mark_dirty(node)
    }

    pub fn style(&self, node: Node) -> Result<&Style, Error> {
        self.style.get(node)
    }

    pub fn layout(&self, node: Node) -> Result<&Layout, Error> {
        self.layout.get(node)
    }

    pub fn mark_dirty(&mut self, node: Node) -> Result<(), Error> {
        fn mark_dirty_impl(
            node: Node,
            layout_cache: &mut Storage<Option<Cache>>,
            is_dirty: &mut Storage<bool>,
            parents: &Storage<Vec<Node>>,
        ) -> Result<(), Error> {
            *layout_cache.get_mut(node)? = None;
            *is_dirty.get_mut(node)? = true;

            for parent in parents.get(node)? {
                mark_dirty_impl(*parent, layout_cache, is_dirty, parents)?;
            }

            Ok(())
        }

        mark_dirty_impl(node, &mut self.layout_cache, &mut self.is_dirty, &self.parents)
    }

    pub fn dirty(&self, node: Node) -> Result<bool, Error> {
        self.is_dirty.get(node).map(|v| *v)
    }

    pub fn compute_layout(&mut self, node: Node, size: Size<Number>) -> Result<(), Error> {
        match self.layout.get(node) {
            Ok(_) => self.compute(node, size).map_err(|err| Error::Measure(err)),
            _ => Err(Error::InvalidNode(node)),
        }
    }
}

impl Drop for Stretch {
    fn drop(&mut self) {
        INSTANCE_ALLOCATOR.lock().unwrap().free(&[self.id]);
    }
}
