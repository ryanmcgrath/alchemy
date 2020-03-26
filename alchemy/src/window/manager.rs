//! Per-platform windows have their own nuances, and typically, their own windowservers.
//! We don't want to take away from that, but we do want to avoid scenarios where things get
//! a bit weird.
//!
//! Consider the following: let's say we have a `Window` instantiated in Rust, and we call
//! `.show()` on it. Then the window drops, on the Rust side. We should probably clean up our side,
//! right?
//!
//! There's also the fact that a user could opt to close a window. If that happens, we want to be
//! able to remove it from our structure... hence this manager that acts as a lightweight interface
//! for managing per-platform Window instances.

use std::sync::{Arc, Mutex};
use crate::window::AppWindow;

/// A struct that provides a Window Manager, via some interior mutability magic.
pub struct WindowManager(Mutex<Vec<Arc<Mutex<AppWindow>>>>);

impl WindowManager {
    /// Creates a new WindowManager instance.
    pub(crate) fn new() -> WindowManager {
        WindowManager(Mutex::new(Vec::with_capacity(1)))
    }

    /// Locks and acquires a new window ID, which our Windows use to loop back for
    /// events and callbacks.
    pub(crate) fn allocate_new_window_id(&self) -> usize {
        let windows = self.0.lock().unwrap();
        windows.len() + 1
    }

    /// Adds an `AppWindow` to this instance.
    pub(crate) fn add(&self, window: Arc<Mutex<AppWindow>>) {
        let mut windows = self.0.lock().unwrap();
        if windows.iter().position(|w| Arc::ptr_eq(&w, &window)).is_none() {
            windows.push(window);
        }
    }

    /// On a `will_close` event, our delegates will loop back here and notify that a window
    /// with x id is closing, and should be removed. The `WindowDelegate` `will_close()` event
    /// is fired here.
    ///
    /// At the end of this, the window drops.
    pub(crate) fn will_close(&self, window_id: usize) {
        let mut windows = self.0.lock().unwrap();
        if let Some(index) = windows.iter().position(|window| {
            let mut w = window.lock().unwrap();

            if w.id == window_id {
                w.delegate.will_close();
                return true;
            }

            false
        }) {
            windows.remove(index);
        }
    }
}
