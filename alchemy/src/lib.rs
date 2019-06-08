//! Alchemy is a Rust GUI framework that implements the React Component lifecycle on top of a 
//! delegate system inspired by those found in AppKit/UIKit. It's backed by native widgets
//! per-platform, but doesn't bind you to any one design style or visual appearance.
//! 
//! CSS support (no cascading) provides a familiar syntax for developers who tend to work on
//! UI/UX projects, and the Component lifecycle is familiar enough to anyone who's touched React.

pub use lazy_static::lazy_static;
use proc_macro_hack::proc_macro_hack;

pub use alchemy_lifecycle::{ComponentKey, text};
pub use alchemy_lifecycle::traits::{
    AppDelegate, Component, Props as ComponentProps, WindowDelegate
};

pub use alchemy_lifecycle::error::Error;
pub use alchemy_lifecycle::rsx::{
    RSX, VirtualNode, VirtualText
};

#[proc_macro_hack(support_nested)]
pub use alchemy_macros::rsx;

#[proc_macro_hack]
pub use alchemy_macros::styles;
pub use alchemy_macros::Props;

pub use alchemy_styles::{Color, styles as style_attributes, SpacedSet, StyleSheet, StylesList};

mod app;
pub use app::App;

pub mod components;
pub use components::{Fragment, Text, View};

pub mod window;
pub use window::Window;
