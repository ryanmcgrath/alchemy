//! Implements the Window API. It attempts to provide a nice, common interface across
//! per-platform Window APIs.

use std::sync::{Arc, Mutex};

use alchemy_lifecycle::{ComponentKey, RENDER_ENGINE};
use alchemy_lifecycle::rsx::RSX;
use alchemy_lifecycle::traits::WindowDelegate;

use crate::{App, SHARED_APP};
use crate::components::View;

#[cfg(feature = "cocoa")]
use alchemy_cocoa::window::{Window as PlatformWindowBridge};

/// AppWindow contains the inner details of a Window. It's guarded by a Mutex on `Window`,
/// and you shouldn't create this yourself, but it's documented here so you can understand what
/// it holds.
pub struct AppWindow {
    pub id: usize,
    pub title: String,
    pub bridge: PlatformWindowBridge,
    pub delegate: Box<WindowDelegate>,
    pub render_key: ComponentKey
}

impl AppWindow {
    /// Re-renders a window. This method calls `render()` on the `WindowDelegate`, patches it into
    /// the root tree node, and then diffs the old (current) tree against a new tree by walking it
    /// and determining what needs to be changed. This also calculates and applies layout and
    /// styling.
    ///
    /// This method is called on the `show` event, and in rare cases can be useful to call
    /// directly.
    pub fn render(&mut self) {
        let children = match self.delegate.render() {
            Ok(opt) => opt,
            Err(e) => {
                eprintln!("Error rendering window! {}", e);
                RSX::None
            }
        };

        match RENDER_ENGINE.diff_and_render_root(&self.render_key, children) {
            Ok(_) => {}
            Err(e) => { eprintln!("Error rendering window! {}", e); }
        }
    }

    /// Renders and calls through to the native platform window show method.
    pub fn show(&mut self) {
        self.render();
        self.bridge.show();
    }

    /// Calls through to the native platform window close method.
    pub fn close(&mut self) {
        self.bridge.close();
    }
}

/// Windows represented... well, a Window. When you create one, you get the Window back. When you
/// show one, a clone of the pointer is added to the window manager, and removed on close.
pub struct Window(pub(crate) Arc<Mutex<AppWindow>>);

impl Window {
    /// Creates a new window.
    pub fn new<S: 'static + WindowDelegate>(title: &str, dimensions: (f64, f64, f64, f64), delegate: S) -> Window {
        let window_id = SHARED_APP.windows.allocate_new_window_id();
        let view = View::default();
        let shared_app_ptr: *const App = &**SHARED_APP;
        let bridge = PlatformWindowBridge::new(window_id, title, dimensions, &view, shared_app_ptr);
        let key = match RENDER_ENGINE.register_root_component(view) {
            Ok(key) => key,
            Err(_e) => { panic!("Uhhhh this really messed up"); }
        };
        
        Window(Arc::new(Mutex::new(AppWindow {
            id: window_id,
            title: title.into(),
            bridge: bridge,
            delegate: Box::new(delegate),
            render_key: key
        })))
    }

    /// Renders a window. By default, a window renders nothing - make sure you implement `render()`
    /// on your `WindowDelegate`. Note that calling `.show()` implicitly calls this for you, so you
    /// rarely need to call this yourself.
    pub fn render(&self) {
        let mut window = self.0.lock().unwrap();
        window.render();
    }

    /// Registers this window with the window manager, renders it, and shows it.
    pub fn show(&self) {
        SHARED_APP.windows.add(self.0.clone());
        let mut window = self.0.lock().unwrap();
        window.show();
    }

    /// Hides a window. On some platforms, this is minimizing... on others, like macOS, it's
    /// actually hiding. On mobile, this shouldn't do anything.
    pub fn hide(&self) {

    }

    /// Closes the window, unregistering it from the window manager in the process and ensuring the
    /// necessary delegate method(s) are fired.
    pub fn close(&self) {
        let window_id = self.0.lock().unwrap().id; 
        SHARED_APP.windows.will_close(window_id);
        let mut window = self.0.lock().unwrap();
        window.close();
    }
}

impl Clone for Window {
    /// Clones a `Window` by cloning the inner `AppWindow`.
    fn clone(&self) -> Window {
        Window(self.0.clone())
    }
}

impl Drop for Window {
    /// When a `Window` is dropped, we want to ensure that it's closed, so we'll silently call
    /// `.close()` to be safe.
    fn drop(&mut self) {
        self.close();
    }
}
