//! Implements the `RSX::VirtualNode` struct, which is a bit of a recursive
//! structure.

use std::fmt::{Display, Debug};

use crate::reconciler::key::ComponentKey;
use crate::rsx::{RSX, Props};
use crate::traits::Component;

/// A VirtualNode is akin to an `Element` in React terms. Here, we provide a way
/// for lazy `Component` instantiation, properties, children and so on.
#[derive(Clone)]
pub struct VirtualNode {
    /// Used in debugging/printing/etc.
    pub tag: &'static str,

    /// `Component` instances are created on-demand, if the reconciler deems it be so. This
    /// is a closure that should return an instance of the correct type.
    pub create_component_fn: fn(key: ComponentKey) -> Box<Component>,

    /// `Props`, which are to be passed to this `Component` at various lifecycle methods. Once 
    /// the reconciler takes ownership of this VirtualNode, these props are moved to a different 
    /// location - thus, you shouldn't rely on them for anything unless you specifically keep 
    /// ownership of a VirtualNode.
    ///
    /// This aspect of functionality may be pulled in a later release if it causes too many issues.
    pub props: Option<Props>,

    /// 
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
