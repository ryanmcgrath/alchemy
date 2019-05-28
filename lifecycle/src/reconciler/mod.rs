//! Implements tree diffing, updating, and so on. Unlike a lot of the VDom implementations 
//! you find littered around the web, this is a bit more ECS-ish, and expects Components to retain 
//! their `ComponentKey` passed in their constructor if they want to update. Doing this 
//! enables us to avoid re-scanning or diffing an entire tree.

use std::sync::Mutex;
use std::error::Error;

use alchemy_styles::THEME_ENGINE;
use alchemy_styles::styles::{Appearance, Dimension, Number, Size, Style};

use crate::traits::Component;
use crate::rsx::{Props, RSX, VirtualNode};

use alchemy_styles::stretch::node::{Node as LayoutNode, Stretch as LayoutStore};

pub mod key;
use key::ComponentKey;

pub mod storage;
use storage::ComponentStore;

pub mod error;
use error::RenderEngineError;

mod instance;
use instance::Instance;

/// This is never actually created, and is here primarily to avoid a circular
/// depedency issue (we can't import the View from alchemy's core crate, since the core crate
/// depends on this crate).

pub struct GenericRootView;
impl Component for GenericRootView {
    fn constructor(key: ComponentKey) -> GenericRootView {
        GenericRootView {}
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

        let mut component_store = self.components.lock().unwrap();
        let component_key = component_store.new_key();
        component_store.insert(component_key, Instance::new("root", Box::new(instance), {
            let mut props = Props::default();
            props.styles = "root".into();
            props
        }, {
            let mut layouts_store = self.layouts.lock().unwrap();
            let style = Style::default();
            Some(layouts_store.new_node(style, vec![])?)
        }))?;

        Ok(component_key)
    }

    /// Rendering the root node is a bit different than rendering or updating other nodes, as we 
    /// never want to unmount it, and the results come from a non-`Component` entity (e.g, a
    /// `Window`). Thus, for this one, we do some manual mucking with what we know is the
    /// root view (a `Window` or such root component would call this with it's registered
    /// `ComponentKey`), and then recurse based on the children.
    pub fn diff_and_render_root(&self, key: ComponentKey, child: RSX) -> Result<(), Box<Error>> {
        let mut component_store = self.components.lock().unwrap();
        let mut layout_store = self.layouts.lock().unwrap();

        let new_root_node = RSX::node("root", |_| {
            Box::new(GenericRootView {})
        }, Props::root(match child {
            RSX::VirtualNode(node) => {
                if node.tag == "Fragment" {
                    node.props.children
                } else {
                    println!("Def here...");
                    vec![RSX::VirtualNode(node)]
                }
            },

            _ => vec![]
        }));

        recursively_diff_tree(key, new_root_node, &mut component_store, &mut layout_store)?;

        let layout_node = {
            let mut root_instance = component_store.get_mut(key)?;
            let layout = root_instance.layout.unwrap();
            let mut style = Style::default();
            style.size = Size {
                width: Dimension::Points(600.),
                height: Dimension::Points(600.)
            };
            layout_store.set_style(layout, style);
            layout
        };

        layout_store.compute_layout(layout_node, Size {
            width: Number::Defined(600.),
            height: Number::Defined(600.)
        })?;
        
        println!("Applying layout...");
        walk_and_apply_styles(key, &mut component_store, &mut layout_store)?;

        Ok(())
    }
}

/// Given two trees, will diff them to see if we need to replace or update. Depending on the
/// result, we'll either recurse down a level, or tear down and build up a new tree. The final
/// parameter on this method, `is_root_entity_view`, should only be passed for `Window` or other
/// such instances, as it instructs us to skip the first level since these ones act different.
fn recursively_diff_tree(
    key: ComponentKey,
    new_tree: RSX,
    component_store: &mut ComponentStore,
    layout_store: &mut LayoutStore
) -> Result<(), Box<Error>> {
    // First we need to determine if this node is being replaced or updated. A replace happens if 
    // two nodes are different types - in this case, we check their tag values. This is also a case
    // where, for instance, if the RSX tag is `::None` or `::VirtualText`, we'll treat it as
    // replacing with nothing.
    let is_replace = match &new_tree {
        RSX::VirtualNode(new_tree) => {
            let old_tree = component_store.get(key)?;
            old_tree.tag != new_tree.tag
        },

        // The algorithm will know below not to recurse if we're trying to diff text or empty
        // values. We return false here to avoid entering the `is_replace` phase; `Component`
        // instances (like <Text />) handle taking the child VirtualText instances and working with
        // them to pass to a native widget.
        _ => false
    };

    if is_replace {
        println!("here, what?!");
        unmount_component_tree(key, component_store, layout_store)?;
        //mount_component_tree(
        return Ok(());
    }

    // At this point, we know it's an update pass. Now we need to do a few things:
    //
    // - Diff our `props` and figure out what actions we can take or shortcut.
    // - Let the `Component` instance determine what it should render.
    // - Recurse into the child trees if necessary.
    let mut old_children = component_store.children(key)?;
    old_children.reverse();

    if let RSX::VirtualNode(mut child) = new_tree {
        for new_child_tree in child.props.children {
            match old_children.pop() {
                // If there's a key in the old children for this position, it's 
                // something we need to update, so let's recurse right back into it.
                Some(old_child_key) => {
                    recursively_diff_tree(
                        old_child_key,
                        new_child_tree,
                        component_store,
                        layout_store
                    )?;
                },

                // If there's no matching old key in this position, then we've got a 
                // new component instance to mount. This part now diverts into the Mount
                // phase.
                None => {
                    if let RSX::VirtualNode(tr33amimustfeelohlol) = new_child_tree {
                        let new_child_key = mount_component_tree(
                            tr33amimustfeelohlol,
                            component_store,
                            layout_store
                        )?;

                        component_store.add_child(key, new_child_key)?;
                        link_layout_nodess(key, new_child_key, component_store, layout_store)?;
                    }
                }
            }
        }
    }

    // Trim the fat. If we still have child nodes after diffing in the new child trees,
    // then they're ones that simply need to be unmounted and dropped.
    if old_children.len() > 0 {
        for child in old_children {
            unmount_component_tree(child, component_store, layout_store)?;
        }
    }

    Ok(())
}

/// Given a new `RSX` tree, a `ComponentStore`, and a `LayoutStore`, will recursively construct the
/// tree, emitting required lifecycle events and persisting values. This happens in an inward-out
/// fashion, which helps avoid unnecessary reflow in environments where it can get tricky.
///
/// This method returns a Result, the `Ok` variant containing a tuple of Vecs. These are the child 
/// Component instances and Layout instances that need to be set in the stores.
fn mount_component_tree(
    tree: VirtualNode,
    component_store: &mut ComponentStore,
    layout_store: &mut LayoutStore
) -> Result<ComponentKey, Box<Error>> {
    let key = component_store.new_key();
    let component = (tree.create_component_fn)(key);
    let is_native_backed = component.has_native_backing_node();
    
    // let state = get_derived_state_from_props()
    let mut instance = Instance::new(tree.tag, component, tree.props, None);
    if is_native_backed {
        let mut style = Style::default();
        THEME_ENGINE.configure_styles_for_keys(&instance.props.styles, &mut style, &mut instance.appearance);
        instance.layout = Some(layout_store.new_node(style, vec![])?);
    }
    
    let rendered = instance.component.render(&instance.props);
    // instance.get_snapshot_before_update()
    println!("Rendered... {}", instance.tag);
    component_store.insert(key, instance)?;

    match rendered {
        Ok(child) => if let RSX::VirtualNode(child) = child {
            // We want to support Components being able to return arbitrary iteratable
            // elements, but... well, it's not quite that simple. Thus we'll offer a <Fragment>
            // tag similar to what React does, which just hoists the children out of it and
            // discards the rest.
            if child.tag == "Fragment" {
                println!("        In Fragment {}", child.props.children.len());
                for child_tree in child.props.children {
                    println!("          > WHAT");
                    if let RSX::VirtualNode(child_tree) = child_tree {
                        let child_key = mount_component_tree(child_tree, component_store, layout_store)?;
                        
                        component_store.add_child(key, child_key)?;
                        if is_native_backed {
                            link_layout_nodess(key, child_key, component_store, layout_store)?;
                        }
                    }
                }
            } else {
                println!("        In regular");
                let child_key = mount_component_tree(child, component_store, layout_store)?;
                component_store.add_child(key, child_key)?;
                if is_native_backed {
                    link_layout_nodess(key, child_key, component_store, layout_store)?;
                }
            }
        } else { println!("WTF"); },

        Err(e) => {
            // return an RSX::VirtualNode(ErrorComponentView) or something?
            /* instance.get_derived_state_from_error(e) */
            // render error state or something I guess?
            /* instance.component_did_catch(e, info) */
            eprintln!("Error rendering: {}", e);
        }
    }

    let instance_lol = component_store.get_mut(key)?;
    instance_lol.component.component_did_mount(&instance_lol.props);

    Ok(key)
}

/// Given a `ComponentKey`, a `ComponentStore`, and a `LayoutStore`, will recursively walk the tree found at
/// said key, emitting required lifecycle events and dropping values. This happens in an inward-out
/// fashion, so deepest nodes/components get destroyed first to ensure that the backing widget tree
/// doesn't get some weird dangling issue.
fn unmount_component_tree(
    key: ComponentKey,
    component_store: &mut ComponentStore,
    layout_store: &mut LayoutStore
) -> Result<Vec<LayoutNode>, Box<Error>> {
    let mut instance = component_store.remove(key)?;
    instance.component.component_will_unmount(&instance.props);
    
    let mut layout_nodes = vec![];
    
    let children = component_store.children(key)?;
    for child in children {
        match unmount_component_tree(child, component_store, layout_store) {
            Ok(mut child_layout_nodes) => {
                if let Some(parent_layout_node) = instance.layout {
                    for node in child_layout_nodes {
                        layout_store.remove_child(parent_layout_node, node)?;
                    }
                } else {
                    layout_nodes.append(&mut child_layout_nodes);
                }
            },

            Err(e) => { eprintln!("Error unmounting a component tree: {}", e); }
        }
    }

    // remove node from backing tree

    Ok(layout_nodes)
}

/// Given a tree, will walk the branches until it finds the next root nodes to connect.
/// While this sounds slow, in practice it rarely has to go far in any direction. This could 
/// potentially be done away with some hoisting magic in the `mount()` recursion, but I couldn't 
/// find a pattern that didn't feel like some utter magic in Rust.
///
/// It might be because I'm writing this at 3AM. Feel free to improve it.
fn link_layout_nodess(
    parent: ComponentKey,
    child: ComponentKey,
    components: &mut ComponentStore,
    layouts: &mut LayoutStore
) -> Result<(), Box<Error>> {
    if let (Ok(parent_instance), Ok(child_instance)) = (components.get(parent), components.get(child)) {
        if let (Some(parent_layout), Some(child_layout)) = (parent_instance.layout, child_instance.layout) {
            layouts.add_child(parent_layout, child_layout)?;
            parent_instance.component.append_child_component(&*child_instance.component);
            return Ok(());
        }
    }

    let children = components.children(child)?;
    for child_key in children {
        link_layout_nodess(parent, child_key, components, layouts)?;
    }

    Ok(())
}

/// Walks the tree and passes necessary Layout and Appearance-based styles to Components so they can
/// update their backing widgets accordingly. This happens after a layout computation, typically.
fn walk_and_apply_styles(
    key: ComponentKey,
    components: &mut ComponentStore,
    layouts: &mut LayoutStore
) -> Result<(), Box<Error>> {
    let instance = components.get_mut(key)?;

    if let Some(layout_key) = instance.layout {
        instance.component.apply_styles(
            &instance.appearance,
            layouts.layout(layout_key)?
        );
    }

    for child in components.children(key)? {
        println!("Nesting");
        walk_and_apply_styles(child, components, layouts)?;
    }

    Ok(())
}
