//! This module holds pieces pertaining to `RSX` element(s), which are lightweight
//! structs that represent how something should be flushed to the screen. Alchemy
//! uses these to build and alter UI; they're typically returned from `render()`
//! methods.

use std::any::Any;
use std::fmt::{Debug, Display};

use alchemy_styles::StylesList;

mod virtual_node;
pub use virtual_node::VirtualNode;

mod virtual_text;
pub use virtual_text::VirtualText;

use crate::reconciler::key::ComponentKey;
use crate::traits::Component;

/// An enum representing the types of nodes that the
/// system can work with. `None`, `VirtualText`, or `VirtualNode`.
pub enum RSX {
    None,
    VirtualText(VirtualText),
    VirtualNode(VirtualNode)
}

impl RSX {
    /// Shorthand method for creating a new `RSX::VirtualNode` instance. Rarely should you call
    /// this yourself; the `rsx! {}` macro handles this for you.
    pub fn node<P: Any + 'static>(
        tag: &'static str,
        styles: StylesList,
        create_fn: fn(key: ComponentKey) -> Box<Component>,
        props: P,
        children: Vec<RSX>
    ) -> RSX {
        RSX::VirtualNode(VirtualNode {
            tag: tag,
            create_component_fn: create_fn,
            styles: styles,
            props: Box::new(props),
            children: children
        })
    }
    
    /// Shorthand method for creating a new `RSX::VirtualText` instance. Rarely should you call
    /// this yourself; the `rsx! {}` and `text!()` macros handle this for you. 
    pub fn text(s: String) -> RSX {
        RSX::VirtualText(VirtualText(s))
    }
}

impl IntoIterator for RSX {
    type Item = RSX;
    type IntoIter = std::vec::IntoIter<RSX>;

    /// Turn a single `RSX` node into an iterable instance.
    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl Display for RSX {
    /// Specialized rendering for displaying RSX nodes.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            RSX::VirtualNode(node) => { std::fmt::Display::fmt(&node, f) },
            RSX::VirtualText(text) => { std::fmt::Display::fmt(&text, f) }
            RSX::None => { Ok(()) }
        }
    }
}

impl Debug for RSX {
    /// Specialized rendering for debugging RSX nodes.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RSX::VirtualNode(node) => { std::fmt::Debug::fmt(&node, f) },
            RSX::VirtualText(text) => { std::fmt::Debug::fmt(&text, f) }
            RSX::None => { Ok(()) }
        }
    }
}
