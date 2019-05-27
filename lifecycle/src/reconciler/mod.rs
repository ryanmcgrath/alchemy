//! Implements tree diffing, and attempts to cache Component instances where
//! possible.

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use std::error::Error;
use std::mem::{discriminant, swap};

use alchemy_styles::THEME_ENGINE;
use alchemy_styles::styles::{Appearance,Dimension, Number, Size, Style};

use crate::traits::Component;
use crate::rsx::{Props, RSX, VirtualNode};

use alchemy_styles::stretch::node::{Node as StyleNode, Stretch as LayoutStore};

pub mod key;
use key::ComponentKey;

pub mod storage;
use storage::ComponentStore;

pub mod error;
use error::RenderEngineError;

// This is never actually created, it's just to satisfy the fact that View
// is defined in the core crate, which we can't import here without creating a 
// circular dependency.
struct StubView;
impl Component for StubView {
    fn constructor(key: ComponentKey) -> StubView {
        StubView {}
    }
}

pub struct RenderEngine {
    queued_state_updates: Mutex<Vec<i32>>,
    components: Mutex<ComponentStore>,
    layouts: Mutex<LayoutStore>
}

impl RenderEngine {
    pub(crate) fn new() -> RenderEngine {
        RenderEngine {
            queued_state_updates: Mutex::new(vec![]),
            components: Mutex::new(ComponentStore::new()),
            layouts: Mutex::new(LayoutStore::new())
        }
    }

    // pub fn queue_update_for(&self, component_ptr: usize, updater: Box<Fn() -> Component + Send + Sync + 'static>) {
    // }

    /// `Window`'s (or anything "root" in nature) need to register with the 
    /// reconciler for things like setState to work properly. When they do so,
    /// they get a key back. When they want to instruct the global `RenderEngine` 
    /// to re-render or update their tree, they pass that key and whatever the new tree 
    /// should be.
    pub fn register_root_component<C: Component + 'static>(&self, instance: C) -> Result<ComponentKey, Box<Error>> {
        // Conceivably, this doesn't NEED to be a thing... but for now it is. If you've stumbled
        // upon here, wayward traveler, in need of a non-native-root-component, please open an
        // issue to discuss. :)
        if !instance.has_native_backing_node() {
            return Err(Box::new(RenderEngineError::InvalidRootComponent {}));
        }

        let layout_key = {
            let style = Style::default();
            let mut layouts = self.layouts.lock().unwrap();
            Some(layouts.new_node(style, vec![])?)
        };

        let mut components = self.components.lock().unwrap();
        let component_key = components.new_node(instance, layout_key, vec![])?;
        Ok(component_key)
    }

    /// Given a key, and a new root tree, will diff the tree structure (position, components, 
    /// attributes and so on), and then queue the changes for application to the backing 
    /// framework tree. As it goes through the tree, if a `Component` at a given position
    /// in the two trees is deemed to be the same, it will move instances from the old tree to 
    /// the new tree before discarding the old tree.
    ///
    /// This calls the necessary component lifecycles per-component.
    pub fn diff_and_render_root(&self, key: &ComponentKey, child: RSX) -> Result<(), Box<Error>> {
        /*
        let mut new_root = RSX::node("root", || {
            Box::new(StubView {})
        }, {
            let mut props = Props::default();
            props.styles = "root".into();
            props
        }, match child {
            RSX::VirtualNode(mut child) => {
                let mut children = vec![];

                if child.tag == "Fragment" {
                    children.append(&mut child.children);
                } else {
                    children.push(RSX::VirtualNode(child));
                }

                children
            },

            // If it's an RSX::None or RSX::VirtualText, we'll just do nothing, as... one 
            // requires nothing, and one isn't supported unless it's inside a <Text> tag, and 
            // we know the root element isn't a <Text> if we're here.
            _ => vec![]
        });

        let mut trees = self.trees.lock().unwrap();
        let (old_root, mut stretch) = trees.remove(key).ok_or_else(|| RenderEngineError::InvalidKey {})?;
        let patched_new_root = diff_and_patch_trees(old_root, new_root, &mut stretch, 0)?;

        if let RSX::VirtualNode(node) = &patched_new_root {
            if let Some(layout_node) = &node.layout_node {
                stretch.compute_layout(*layout_node, Size {
                    width: Number::Defined(600.),
                    height: Number::Defined(600.),
                })?;
                walk_and_apply_styles(node, &mut stretch)?;
            }
        }

        trees.insert(*key, (patched_new_root, stretch));*/
        Ok(())
    }
}
