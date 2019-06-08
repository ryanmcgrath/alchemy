//! Implements the Window API. It attempts to provide a nice, common interface across
//! per-platform Window APIs.

use std::rc::Rc;
use std::cell::RefCell;

use alchemy_lifecycle::{ComponentKey, RENDER_ENGINE};
use alchemy_lifecycle::rsx::RSX;
use alchemy_lifecycle::traits::{Component, WindowDelegate};

use alchemy_styles::{Appearance, Style, StylesList, THEME_ENGINE};

use crate::components::View;

#[cfg(feature = "cocoa")]
use alchemy_cocoa::window::{Window as PlatformWindowBridge};

#[cfg(feature = "gtkrs")]
use alchemy_gtkrs::window::{Window as PlatformWindowBridge};

pub struct AppWindow {
    pub styles: StylesList,
    pub title: String,
    pub dimensions: (f64, f64, f64, f64),
    pub bridge: Option<PlatformWindowBridge>,
    pub delegate: Box<WindowDelegate>,
    render_key: Option<ComponentKey>
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
        let mut style = Style::default();
        let mut appearance = Appearance::default();
        THEME_ENGINE.configure_styles_for_keys(&self.styles, &mut style, &mut appearance);

        if let Some(bridge) = &mut self.bridge {
            bridge.apply_styles(&appearance);
        }

        let children = match self.delegate.render() {
            Ok(opt) => opt,
            Err(e) => {
                eprintln!("Error rendering window! {}", e);
                RSX::None
            }
        };

        if let Some(render_key) = self.render_key {
            match RENDER_ENGINE.diff_and_render_root(render_key, (
                self.dimensions.2,
                self.dimensions.3
            ), children) {
                Ok(_) => { }
                Err(e) => { eprintln!("Error rendering window! {}", e); }
            }
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.into();
        if let Some(bridge) = &mut self.bridge {
            bridge.set_title(title);
        }
    }

    pub fn set_dimensions(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.dimensions = (x, y, width, height);
        if let Some(bridge) = &mut self.bridge {
            bridge.set_dimensions(x, y, width, height);
        }
    }

    /// Renders and calls through to the native platform window show method.
    pub fn show(&mut self) {
        self.render();
        if let Some(bridge) = &mut self.bridge {
            bridge.show();
        }
    }

    /// Calls through to the native platform window close method.
    pub fn close(&mut self) {
        if let Some(bridge) = &mut self.bridge {
            bridge.close();
        }
    }
}

impl WindowDelegate for AppWindow {}

/// Window represents... well, a Window. When you create one, you get the Window back.
pub struct Window(pub Rc<RefCell<AppWindow>>);

impl Window {
    /// Creates a new window.
    pub fn new<S: 'static + WindowDelegate>(delegate: S) -> Window {        
        let app_window = Rc::new(RefCell::new(AppWindow {
            styles: "".into(),
            title: "".into(),
            dimensions: (0., 0., 0., 0.),
            bridge: None,
            delegate: Box::new(delegate),
            render_key: None
        }));
        
        let app_ptr: *const RefCell<AppWindow> = &*app_window;
        {
            // This unwrap() is fine, since we implement View ourselves in Alchemy
            let view = View::default();
            let backing_node = view.borrow_native_backing_node().unwrap();

            let mut window = app_window.borrow_mut();
            window.bridge = Some(PlatformWindowBridge::new(backing_node, app_ptr));
            window.render_key = match RENDER_ENGINE.register_root_component(view) {
                Ok(key) => Some(key),
                Err(_e) => { panic!("Uhhhh this really messed up"); }
            };
        }

        Window(app_window)
    }

    /// Renders a window. By default, a window renders nothing - make sure you implement `render()`
    /// on your `WindowDelegate`. Note that calling `.show()` implicitly calls this for you, so you
    /// rarely need to call this yourself.
    pub fn render(&self) {
        let window = self.0.borrow_mut();
        window.render();
    }

    pub fn set_title(&self, title: &str) {
        let mut window = self.0.borrow_mut();
        window.set_title(title);
    }

    pub fn set_dimensions(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let mut window = self.0.borrow_mut();
        window.set_dimensions(x, y, width, height);
    }

    /// Registers this window with the window manager, renders it, and shows it.
    pub fn show(&self) {
        let mut window = self.0.borrow_mut();
        window.show();
    }

    /// Hides a window. On some platforms, this is minimizing... on others, like macOS, it's
    /// actually hiding. On mobile, this shouldn't do anything.
    pub fn hide(&self) {

    }

    /// Closes the window, unregistering it from the window manager in the process and ensuring the
    /// necessary delegate method(s) are fired.
    pub fn close(&self) {
        let mut window = self.0.borrow_mut();
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
