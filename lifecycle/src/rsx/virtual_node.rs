//! Implements the `RSX::VirtualNode` struct, which is a bit of a recursive
//! structure.

use std::sync::{Arc, RwLock};
use std::fmt::{Display, Debug};

use alchemy_styles::node::Node;

use crate::traits::Component;
use crate::rsx::{RSX, Props};

/// A VirtualNode is akin to an `Element` in React terms. Here, we provide a way
/// for lazy `Component` instantiation, along with storage for things like layout nodes,
/// properties, children and so on.
#[derive(Clone)]
pub struct VirtualNode {
    /// Used in debugging/printing/etc.
    pub tag: &'static str,

    /// `Component` instances are created on-demand, if the reconciler deems it be so. This
    /// is a closure that should return an instance of the correct type.
    pub create_component_fn: fn() -> Arc<RwLock<Component>>,

    /// A cached component instance, which is transferred between trees. Since `Component` 
    /// instances are lazily created, this is an `Option`, and defaults to `None`.
    pub instance: Option<Arc<RwLock<Component>>>,

    /// A cached `Node` for computing `Layout` with `Stretch`. Some components may not have
    /// a need for layout (e.g, if they don't have a backing node), and thus this is optional.
    ///
    /// The reconciler will handle bridging tree structures as necessary.
    pub layout_node: Option<Node>,

    /// `Props`, which are to be passed to this `Component` at various lifecycle methods.
    pub props: Props,

    /// Computed children get stored here.
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
