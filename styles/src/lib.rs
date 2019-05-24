//! This crate hoists various styles and layout parameters for implementing
//! Flexbox in Alchemy. For all intents and purposes, you can essentially consider
//! this to be the root crate for Alchemy, as just about everything ends up using it.

#[cfg(feature="parser")]
#[macro_use] pub extern crate cssparser;

mod stretch;
pub use stretch::{geometry, node, number, result, Stretch, Error};

pub mod color;
pub mod styles;

#[cfg(feature="parser")]
pub mod styles_parser;
