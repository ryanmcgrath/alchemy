//! Alchemy is a Rust GUI framework that implements the React Component lifecycle on top of a 
//! delegate system inspired by those found in AppKit/UIKit. It's backed by native widgets
//! per-platform, but doesn't bind you to any one design style or visual appearance.
//! 
//! CSS support (no cascading) provides a familiar syntax for developers who tend to work on
//! UI/UX projects, and the Component lifecycle is familiar enough to anyone who's touched React.

use std::sync::Arc;
pub use lazy_static::lazy_static;
use proc_macro_hack::proc_macro_hack;

pub use alchemy_lifecycle::traits::{
    AppDelegate, Component, WindowDelegate
};

pub use alchemy_lifecycle::error::Error;
pub use alchemy_lifecycle::rsx::{
    Props, RSX, StyleKey, StylesList,
    SpacedSet, VirtualNode, VirtualText
};

#[proc_macro_hack(support_nested)]
pub use alchemy_macros::rsx;

#[proc_macro_hack]
pub use alchemy_macros::styles;

mod app;
use app::App;

pub mod components;
pub use components::{Fragment, View};

pub(crate) mod reconciler;
pub mod theme;

pub mod window;
pub use window::Window;

lazy_static! {
    pub(crate) static ref SHARED_APP: Arc<App> = App::new();
}

/// This function supports calling the shared global application instance from anywhere in your
/// code. It's useful in cases where you need to have an escape hatch, but if you're using it as
/// such, you may want to double check your Application design to make sure you need it.
pub fn shared_app() -> Arc<App> {
    SHARED_APP.clone()
}