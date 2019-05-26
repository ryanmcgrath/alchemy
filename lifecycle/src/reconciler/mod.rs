//! Implements tree diffing, and attempts to cache Component instances where
//! possible.

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use std::error::Error;
use std::mem::{discriminant, swap};

use uuid::Uuid;

use alchemy_styles::{Stretch, THEME_ENGINE};
use alchemy_styles::styles::{Style, Dimension};
use alchemy_styles::number::Number;
use alchemy_styles::geometry::Size;

use crate::traits::Component;
use crate::rsx::{Props, RSX, VirtualNode};

mod error;
use error::RenderEngineError;

// This is never actually created, it's just to satisfy the fact that View
// is defined in the core crate, which we can't import here without creating a 
// circular dependency.
struct StubView;
impl Component for StubView {}

pub struct RenderEngine {
    pending_state_updates: Mutex<Vec<i32>>,
    trees: Mutex<HashMap<Uuid, (RSX, Stretch)>>
}

impl RenderEngine {
    pub(crate) fn new() -> RenderEngine {
        RenderEngine {
            pending_state_updates: Mutex::new(vec![]),
            trees: Mutex::new(HashMap::new())
        }
    }

    /// `Window`'s (or anything "root" in nature) need to register with the 
    /// reconciler for things like setState to work properly. When they do so,
    /// they get a key back. When they want to instruct the global `RenderEngine` 
    /// to re-render or update their tree, they pass that key and whatever the new tree 
    /// should be.
    pub fn register_root_component<C: Component + 'static>(&self, instance: C) -> Uuid {
        let mut root_node = RSX::node("root", || {
            Arc::new(RwLock::new(StubView {}))
        }, {
            let mut props = Props::default();
            props.styles = "root".into();
            props
        });
    
        let mut stretch = Stretch::new();
        if let RSX::VirtualNode(root) = &mut root_node {
            let mut style = Style::default();
            style.size = Size {
                width: Dimension::Points(600.),
                height: Dimension::Points(600.)
            };
            
            root.instance = Some(Arc::new(RwLock::new(instance)));
            root.layout_node = match stretch.new_node(style, vec![]) {
                Ok(node) => Some(node),
                Err(e) => { None }
            }
        }
        
        let key = Uuid::new_v4();
        let mut trees = self.trees.lock().unwrap();
        trees.insert(key, (root_node, stretch));
        key
    }

    /// Given a key, and a new root tree, will diff the tree structure (position, components, 
    /// attributes and so on), and then queue the changes for application to the backing 
    /// framework tree. As it goes through the tree, if a `Component` at a given position
    /// in the two trees is deemed to be the same, it will move instances from the old tree to 
    /// the new tree before discarding the old tree.
    ///
    /// This calls the necessary component lifecycles per-component.
    pub fn diff_and_render_root(&self, key: &Uuid, child: RSX) -> Result<(), Box<Error>> {
        let mut new_root = RSX::node("root", || {
            Arc::new(RwLock::new(StubView {}))
        }, {
            let mut props = Props::default();
            props.styles = "root".into();
            props
        });

        // If it's an RSX::None, or a RSX::VirtualText, we do nothing, as... one
        // requires nothing, and one isn't supported unless it's inside a <Text> tag. 
        if let RSX::VirtualNode(mut child) = child {
            if let RSX::VirtualNode(new_root_node) = &mut new_root {
                if child.tag == "Fragment" {
                    new_root_node.children.append(&mut child.children);
                } else {
                    new_root_node.children.push(RSX::VirtualNode(child));
                }
            }
        }

        let mut trees = self.trees.lock().unwrap();
        let (old_root, mut stretch) = trees.remove(key).ok_or_else(|| RenderEngineError::InvalidKeyError {})?;
        let patched_new_root = diff_and_patch_trees(old_root, new_root, &mut stretch, 0)?;

        if let RSX::VirtualNode(node) = &patched_new_root {
            walk_and_apply_styles(node, &mut stretch)?;
        }

        trees.insert(*key, (patched_new_root, stretch));
        println!("RENDERED");
        Ok(())
    }
}

/// Given two node trees, will compare, diff, and apply changes in a recursive fashion. 
pub fn diff_and_patch_trees(old: RSX, new: RSX, stretch: &mut Stretch, depth: usize) -> Result<RSX, Box<Error>> {
    // Whether we replace or not depends on a few things. If we're working on two different node
    // types (text vs node), if the node tags are different, or if the key (in some cases) is
    // different.
    let is_replace = match discriminant(&old) != discriminant(&new) {
        true => true,
        false => {
            if let (RSX::VirtualNode(old_element), RSX::VirtualNode(new_element)) = (&old, &new) {
                old_element.tag != new_element.tag
            } else {
                false
            }
        }
    };
    
    match (old, new) {
        (RSX::VirtualNode(mut old_element), RSX::VirtualNode(mut new_element)) => {
            if is_replace {
                // Do something different in here...
                //let mut mounted = mount_component_tree(new_tree); 
                // unmount_component_tree(old_tree);
                // Swap them in memory, copy any layout + etc as necessary
                // append, link layout nodes, etc
                return Ok(RSX::VirtualNode(new_element));
            }

            // If we get here, it's an update to an existing element. This means a cached Component
            // instance might exist, and we want to keep it around and reuse it if possible. Let's check
            // and do some swapping action to handle it.
            //
            // These need to move to the new tree, since we always keep 'em. We also wanna cache a
            // reference to our content view.
            swap(&mut old_element.instance, &mut new_element.instance);
            swap(&mut old_element.layout_node, &mut new_element.layout_node);

            // For the root tag, which is usually the content view of the Window, we don't want to
            // perform the whole render/component lifecycle routine. It's a special case element,
            // where the Window (or other root element) patches in the output of a render method
            // specific to that object. An easy way to handle this is the depth parameter - in
            // fact, it's why it exists. Depth 0 should be considered special and skip the
            // rendering phase.
            if depth > 0 {
                // diff props, set new props
                // instance.get_derived_state_from_props()
                
                if let Some(instance) = &mut new_element.instance {
                    // diff props, set new props
                    // instance.get_derived_state_from_props()
                    
                    //if instance.should_component_update() {
                        // instance.render() { }
                        // instance.get_snapshot_before_update()
                        // apply changes
                        //instance.component_did_update();
                    //} else {
                        // If should_component_update() returns false, then we want to take the
                        // children from the old node, move them to the new node, and recurse into
                        // that tree instead.
                    //}
                }
            }

            // This None path should never be hit, we just need to use a rather verbose pattern
            // here. It's unsightly, I know.
            let is_native_backed = match &new_element.instance {
                Some(instance) => {
                    let lock = instance.read().unwrap();
                    lock.has_native_backing_node()
                },
                None => false
            };

            // There is probably a nicer way to do this that doesn't allocate as much, and I'm open
            // to revisiting it. Platforms outside of Rust allocate far more than this, though, and
            // in general the whole "avoid allocations" thing is fear mongering IMO. Revisit later.
            //
            // tl;dr we allocate a new Vec<RSX> that's equal to the length of our new children, and
            // then swap it on our (owned) node... it's safe, as we own it. This allows us to
            // iterate and dodge the borrow checker.
            let mut children: Vec<RSX> = Vec::with_capacity(new_element.children.len());
            std::mem::swap(&mut children, &mut new_element.children);
            
            old_element.children.reverse();
            for new_child_tree in children {
                match old_element.children.pop() {
                    // A matching child in the old tree means we can recurse right back into the
                    // update phase.
                    Some(old_child_tree) => {
                        let updated = diff_and_patch_trees(old_child_tree, new_child_tree, stretch, depth + 1)?;
                        new_element.children.push(updated);
                    },

                    // If there's no matching child in the old tree, this is a new Component and we
                    // can feel free to mount/connect it.
                    None => {
                        if let RSX::VirtualNode(new_el) = new_child_tree {
                            let mut mounted = mount_component_tree(new_el, stretch)?;
                            
                            // Link the layout nodes, handle the appending, etc.
                            // This happens inside mount_component_tree, but that only handles that
                            // specific tree. Think of this step as joining two trees in the graph.
                            if is_native_backed {
                                find_and_link_layout_nodes(&mut new_element, &mut mounted, stretch)?;
                            }
                            
                            new_element.children.push(RSX::VirtualNode(mounted));
                        }
                    }
                }
            }
            
            // Trim the fat - more children in the old tree than the new one means we gonna be
            // droppin'. We need to send unmount lifecycle calls to these, and break any links we
            // have (e.g, layout, backing view tree, etc).
            loop {
                match old_element.children.pop() {
                    Some(child) => {
                        if let RSX::VirtualNode(mut old_child) = child {
                            unmount_component_tree(&mut old_child, stretch)?;
                        }
                    },

                    None => { break; }
                }
            }

            Ok(RSX::VirtualNode(new_element))
        }
        
        // We're comparing two text nodes. Realistically... this requires nothing from us, because
        // the <Text> tag (or any other component instance, if it desires) should handle it.
        (RSX::VirtualText(_), RSX::VirtualText(text)) => {
            Ok(RSX::VirtualText(text))
        }

        // These are all edge cases that shouldn't get hit. In particular:
        //
        //  - VirtualText being replaced by VirtualNode should be caught by the discriminant check
        //      in the beginning of this function, which registers as a replace/mount.
        //  - VirtualNode being replaced with VirtualText is the same scenario as above.
        //  - The (RSX::None, ...) checks are to shut the compiler up; we never store the RSX::None
        //      return value, as it's mostly a value in place for return signature usability. Thus,
        //      these should quite literally never register.
        //
        //  This goes without saying, but: never ever store RSX::None lol
        (RSX::VirtualText(_), RSX::VirtualNode(_)) | (RSX::VirtualNode(_), RSX::VirtualText(_)) |
        (RSX::None, RSX::VirtualText(_)) | (RSX::None, RSX::VirtualNode(_)) | (RSX::None, RSX::None) |
        (RSX::VirtualNode(_), RSX::None) | (RSX::VirtualText(_), RSX::None) => {
            unreachable!("Unequal variant discriminants should already have been handled.");
        }
    }
}

/// Walks the tree and applies styles. This happens after a layout computation, typically.
pub(crate) fn walk_and_apply_styles(node: &VirtualNode, layout_manager: &mut Stretch) -> Result<(), Box<Error>> {
    if let (Some(layout_node), Some(instance)) = (node.layout_node, &node.instance) {
        let component = instance.write().unwrap();
        component.apply_styles(
            layout_manager.layout(layout_node)?,
            layout_manager.style(layout_node)?
        );
    }

    for child in &node.children {
        if let RSX::VirtualNode(child_node) = child {
            walk_and_apply_styles(child_node, layout_manager)?;
        }
    }

    Ok(())
}

/// Given a tree, will walk the branches until it finds the next root nodes to connect.
/// While this sounds slow, in practice it rarely has to go far in any direction.
fn find_and_link_layout_nodes(parent_node: &mut VirtualNode, child_tree: &mut VirtualNode, stretch: &mut Stretch) -> Result<(), Box<Error>> {
    if let (Some(parent_instance), Some(child_instance)) = (&mut parent_node.instance, &mut child_tree.instance) {
        if let (Some(parent_layout_node), Some(child_layout_node)) = (&parent_node.layout_node, &child_tree.layout_node) {
            stretch.add_child(*parent_layout_node, *child_layout_node)?;
            
            let parent_component = parent_instance.write().unwrap();
            let child_component = child_instance.read().unwrap();
            parent_component.append_child_component(&*child_component);
            
            return Ok(());
        }
    }

    for child in child_tree.children.iter_mut() {
        if let RSX::VirtualNode(child_tree) = child {
            find_and_link_layout_nodes(parent_node, child_tree, stretch)?;
        }
    }

    Ok(())
}

/// Recursively constructs a Component tree. This entails adding it to the backing
/// view tree, firing various lifecycle methods, and ensuring that nodes for layout
/// passes are configured.
///
/// In the future, this would ideally return patch-sets for the backing layer or something.
fn mount_component_tree(mut new_element: VirtualNode, stretch: &mut Stretch) -> Result<VirtualNode, Box<Error>> {
    let instance = (new_element.create_component_fn)();

    let mut is_native_backed = false;
    
    let rendered = {
        let component = instance.read().unwrap();
        // instance.get_derived_state_from_props(props)

        is_native_backed = component.has_native_backing_node();

        if is_native_backed {
            let mut style = Style::default();
            THEME_ENGINE.configure_style_for_keys(&new_element.props.styles, &mut style);
            
            let layout_node = stretch.new_node(style, vec![])?;
            new_element.layout_node = Some(layout_node);
        }
        
        component.render(&new_element.props)
    };
    
    // instance.get_snapshot_before_update()

    new_element.instance = Some(instance);

    let mut children = match rendered {
        Ok(opt) => match opt {
            RSX::VirtualNode(child) => {
                let mut children = vec![];
                
                // We want to support Components being able to return arbitrary iteratable
                // elements, but... well, it's not quite that simple. Thus we'll offer a <Fragment>
                // tag similar to what React does, which just hoists the children out of it and
                // discards the rest.
                if child.tag == "Fragment" {
                    for child_node in child.props.children {
                        if let RSX::VirtualNode(node) = child_node {
                            let mut mounted = mount_component_tree(node, stretch)?;
                            
                            if is_native_backed {
                                find_and_link_layout_nodes(&mut new_element, &mut mounted, stretch)?;
                            }

                            children.push(RSX::VirtualNode(mounted)); 
                        }
                    }
                } else {
                    let mut mounted = mount_component_tree(child, stretch)?;
                    
                    if is_native_backed {
                        find_and_link_layout_nodes(&mut new_element, &mut mounted, stretch)?;
                    }

                    children.push(RSX::VirtualNode(mounted));
                }
                
                children
            },

            // If a Component renders nothing (or this is a Text string, which we do nothing with)
            // that's totally fine.
            _ => vec![]
        },

        Err(e) => {
            // return an RSX::VirtualNode(ErrorComponentView) or something?
            /* instance.get_derived_state_from_error(e) */
            // render error state or something I guess?
            /* instance.component_did_catch(e, info) */
            eprintln!("Error rendering: {}", e);
            vec![]
        }
    };

    new_element.children.append(&mut children);
    
    if let Some(instance) = &mut new_element.instance {
        let mut component = instance.write().unwrap();
        component.component_did_mount(&new_element.props);
    }
    
    Ok(new_element)
}

/// Walk the tree and unmount Component instances. This means we fire the
/// `component_will_unmount` hook and remove the node(s) from their respective trees.
///
/// This fires the hooks from a recursive inward-out pattern; that is, the deepest nodes in the tree
/// are the first to go, ensuring that everything is properly cleaned up.
fn unmount_component_tree(old_element: &mut VirtualNode, stretch: &mut Stretch) -> Result<(), Box<Error>> {
    // We only need to recurse on VirtualNodes. Text and so on will automagically drop
    // because we don't support freeform text, it has to be inside a <Text> at all times.
    for child in old_element.children.iter_mut() {
        if let RSX::VirtualNode(child_element) = child {
            unmount_component_tree(child_element, stretch)?;
        }
    }

    // Fire the appropriate lifecycle method and then remove the node from the underlying
    // graph. Remember that a Component can actually not necessarily have a native backing
    // node, hence our necessary check.
    if let Some(old_component) = &mut old_element.instance {
        let mut component = old_component.write().unwrap();
        component.component_will_unmount(&old_element.props);

        /*if let Some(view) = old_component.get_native_backing_node() {
            if let Some(native_view) = replace_native_view {
                //replace_view(&view, &native_view);
            } else {
                //remove_view(&view);
            }
        }*/
    }

    // Rather than try to keep track of parent/child stuff for removal... just obliterate it,
    // the underlying library does a good job of killing the links anyway.
    if let Some(layout_node) = &mut old_element.layout_node {
        stretch.set_children(*layout_node, vec![])?;
    }

    Ok(())
}

/*let mut add_attributes: HashMap<&str, &str> = HashMap::new();
let mut remove_attributes: Vec<&str> = vec![];

// TODO: -> split out into func
for (new_attr_name, new_attr_val) in new_element.attrs.iter() {
    match old_element.attrs.get(new_attr_name) {
        Some(ref old_attr_val) => {
            if old_attr_val != &new_attr_val {
                add_attributes.insert(new_attr_name, new_attr_val);
            }
        }
        None => {
            add_attributes.insert(new_attr_name, new_attr_val);
        }
    };
}

// TODO: -> split out into func
for (old_attr_name, old_attr_val) in old_element.attrs.iter() {
    if add_attributes.get(&old_attr_name[..]).is_some() {
        continue;
    };

    match new_element.attrs.get(old_attr_name) {
        Some(ref new_attr_val) => {
            if new_attr_val != &old_attr_val {
                remove_attributes.push(old_attr_name);
            }
        }
        None => {
            remove_attributes.push(old_attr_name);
        }
    };
}

if add_attributes.len() > 0 {
    patches.push(Patch::AddAttributes(*cur_node_idx, add_attributes));
}
if remove_attributes.len() > 0 {
    patches.push(Patch::RemoveAttributes(*cur_node_idx, remove_attributes));
}*/
