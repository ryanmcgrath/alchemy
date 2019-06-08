//! Implements a `gtk::ApplicationWindow` wrapper for GTK-based systems.
//! This also handles looping back lifecycle events, such as window 
//! resizing or close events.

use std::cell::RefCell;

use gtk::{
    ContainerExt,
    GtkWindowExt, WidgetExt,
    Window as GtkWindow, WindowType
};

use alchemy_lifecycle::traits::WindowDelegate;
use alchemy_styles::Appearance;

/// A wrapper for `NSWindow`. Holds (retains) pointers for the Objective-C runtime 
/// where our `NSWindow` and associated delegate live.
pub struct Window {
    pub inner: GtkWindow
}

impl Window {
    /// Creates a new `NSWindow` instance, configures it appropriately (e.g, titlebar appearance),
    /// injects an `NSObject` delegate wrapper, and retains the necessary Objective-C runtime
    /// pointers.
    pub fn new<T: WindowDelegate>(content_view: (), app_ptr: *const RefCell<T>) -> Window {
        Window {
            inner: GtkWindow::new(WindowType::Toplevel)
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.inner.set_title(title);
    }

    pub fn set_dimensions(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.inner.set_position(gtk::WindowPosition::Center);
        self.inner.set_default_size(width as i32, height as i32);
    }

    /// Normally used for setting platform-specific styles; on macOS we choose not to do this and
    /// just have the content view handle the background color, as calling window
    /// setBackgroundColor causes some notable lag on resizing.
    pub fn apply_styles(&mut self, _appearance: &Appearance) { }

    /// On macOS, calling `show()` is equivalent to calling `makeKeyAndOrderFront`. This is the
    /// most common use case, hence why this method was chosen - if you want or need something
    /// else, feel free to open an issue to discuss.
    ///
    /// You should never be calling this yourself, mind you - Alchemy core handles this for you.
    pub fn show(&self) {
        let button = gtk::Button::new_with_label("Click me!");
        self.inner.add(&button);
        self.inner.show_all();
    }

    /// On macOS, calling `close()` is equivalent to calling... well, `close`. It closes the
    /// window.
    ///
    /// I dunno what else to say here, lol.
    ///
    /// You should never be calling this yourself, mind you - Alchemy core handles this for you.
    pub fn close(&self) {
    }
}

impl Drop for Window {
    /// When a Window is dropped on the Rust side, we want to ensure that we break the delegate
    /// link on the Objective-C side. While this shouldn't actually be an issue, I'd rather be
    /// safer than sorry.
    fn drop(&mut self) {
    }
}
