//! Implements a set of Error types that could happen during a diff/patch/reflow 
//! run. These are mostly internal to the rendering engine itself, but could potentially 
//! show up elsewhere.

use std::fmt;
use std::error::Error;

pub enum RenderEngineError {
    InvalidKeyError
}

impl fmt::Display for RenderEngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RenderEngineError::InvalidKeyError => write!(f, "An invalid key was passed to the render engine.")
        }
    }
}

impl fmt::Debug for RenderEngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RenderEngineError::InvalidKeyError => write!(f, "An invalid key was passed to the render engine: {{ file: {}, line: {} }}", file!(), line!())
        }
    }
}

impl Error for RenderEngineError {

}
