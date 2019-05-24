//! This module implements Windows and their associated lifecycles.

mod manager;
pub(crate) use manager::WindowManager;

pub mod window;
pub use window::{AppWindow, Window};
