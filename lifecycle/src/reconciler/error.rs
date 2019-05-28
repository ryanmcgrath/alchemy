//! Implements a set of Error types that could happen during a diff/patch/reflow 
//! run. These are mostly internal to the rendering engine itself, but could potentially 
//! show up elsewhere.

use crate::reconciler::key::ComponentKey;

#[derive(Debug)]
pub enum RenderEngineError {
    InvalidKey,
    InvalidRootComponent,
    InvalidComponentKey(ComponentKey)
}

impl std::fmt::Display for RenderEngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RenderEngineError::InvalidComponentKey(ref node) => write!(f, "Invalid component key {:?}", node),
            RenderEngineError::InvalidRootComponent => write!(f, "Invalid component type! Root nodes must be a natively backed node."),
            RenderEngineError::InvalidKey => write!(f, "An invalid key was passed to the render engine.")
        }
    }
}

impl std::error::Error for RenderEngineError {
    fn description(&self) -> &str {
        match *self {
            RenderEngineError::InvalidComponentKey(_) => "The key is not part of the component storage instance",
            RenderEngineError::InvalidRootComponent => "The root component must be a natively backed Component instance.",
            RenderEngineError::InvalidKey => "An invalid key was passed to the render engine."
        }
    }
}
