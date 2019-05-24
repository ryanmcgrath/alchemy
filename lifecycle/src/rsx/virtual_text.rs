//! Implements `RSX::VirtualText`, which holds data pertaining to <Text>, primarily.

use std::fmt::{Display, Debug};

/// Currently a wrapper for `String`, but could be something else down the road. Frees
/// us from needing to change the public API later.
#[derive(Clone)]
pub struct VirtualText(pub String);

impl VirtualText {
    /// Given a `String`, returns a `VirtualText` node.
    pub fn new(s: String) -> VirtualText {
        VirtualText(s)
    }
}

impl Display for VirtualText {
    /// Formatting for `VirtualText` display.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl Debug for VirtualText {
    /// Formatting for `VirtualText` debugging.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VirtualText({})", self.0)
    }
}
