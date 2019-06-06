//! Implements the `RSX::VirtualNode` struct, which is a bit of a recursive
//! structure.

use std::any::Any;
use std::fmt::{Display, Debug};

use alchemy_styles::StylesList;

use crate::reconciler::key::ComponentKey;
use crate::rsx::RSX;
use crate::traits::Component;

/// A VirtualNode is akin to an `Element` in React terms. Here, we provide a way
/// for lazy `Component` instantiation, properties, children and so on.
pub struct VirtualNode {
    /// Used in debugging/printing/etc.
    pub tag: &'static str,

    /// Used for determining which CSS styles should be applied to this node.
    /// This property is accessed often enough that it's separated out here.
    pub styles: StylesList,

    /// `Component` instances are created on-demand, if the reconciler deems it be so. This
    /// is a closure that should return an instance of the correct type.
    pub create_component_fn: fn(key: ComponentKey) -> Box<Component>,

    /// When some RSX is returned, we scoop up the props inside a special block, and then shove
    /// them in here as an `Any` object. When you `derive(Props)` on a `Component` struct, it 
    /// creates a setter that specifically handles downcasting and persisting props for you.
    pub props: Box<Any>,

    /// Child components for this node.
    pub children: Vec<RSX>
}

impl Display for VirtualNode {
    /// Special formatting for displaying nodes.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "<{}>", self.tag)?;

        for child in &self.children {
            write!(f, "{:?}", child)?;
        }

        write!(f, "</{}>", self.tag)
    }
}

impl Debug for VirtualNode {
    /// Special formatting for debugging nodes.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VirtualNode({})", self.tag)
    }
}
