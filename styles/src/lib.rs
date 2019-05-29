//! This crate hoists various styles and layout parameters for implementing
//! Flexbox in Alchemy. For all intents and purposes, you can essentially consider
//! this to be the root crate for Alchemy, as just about everything ends up using it.

// We hoist this for ease of use in other crates, since... well, pretty much
// every other crate in the project imports this already.
pub use lazy_static::lazy_static;

#[cfg(feature="parser")]
#[macro_use] pub extern crate cssparser;

pub mod color;
pub use color::Color;

mod engine;
use engine::ThemeEngine;

mod spacedlist;
pub use spacedlist::SpacedList;

mod spacedset;
pub use spacedset::SpacedSet;

pub mod stretch;
pub use stretch::result::Layout;

mod style_keys;
pub use style_keys::StyleKey;
pub type StylesList = SpacedSet<StyleKey>;

pub mod styles;
pub use styles::{Appearance, Styles, Style};

pub mod stylesheet;
pub use stylesheet::StyleSheet;

#[cfg(feature="parser")]
pub mod styles_parser;

lazy_static! {
    pub static ref THEME_ENGINE: ThemeEngine = ThemeEngine::new();
}
