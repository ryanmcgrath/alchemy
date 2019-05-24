//! This module implements the Application structure and associated 
//! lifecycle methods. You typically never create this struct yourself; 
//! in Alchemy, there's a global `shared_app` that you should use to work
//! with the `App` struct.
//!
//! This ensures that you can respond to application lifecycles, and so
//! routing things around works correctly.

use std::sync::{Arc, Mutex};

use alchemy_lifecycle::traits::AppDelegate;

use crate::theme::{ThemeEngine, StyleSheet};
use crate::window::WindowManager;

#[cfg(feature = "cocoa")]
pub use alchemy_cocoa::app::{App as PlatformAppBridge};

/// A default delegate that is mostly used for creating the initial struct,
/// without requiring the actual `AppDelegate` from the user. Will ideally
/// never see the light of day.
struct DefaultAppDelegate;
impl AppDelegate for DefaultAppDelegate {}

/// The Application structure itself. It holds a Mutex'd platform bridge, to
/// handle communicating with the platform-specific app instance, along with a
/// delegate to forward events to. The `ThemeEngine` and `WindowManager` are
/// also stored here for easy access.
pub struct App {
    pub(crate) bridge: Mutex<Option<PlatformAppBridge>>,
    pub delegate: Mutex<Box<AppDelegate>>,
    pub themes: ThemeEngine,
    pub windows: WindowManager
}

impl App {
    /// Creates a new app, allocated on the heap. Provides a pointer to
    /// said allocated instance so that the platform-specific app instances
    /// can loop events back around.
    pub(crate) fn new() -> Arc<App> {
        let app = Arc::new(App {
            bridge: Mutex::new(None),
            delegate: Mutex::new(Box::new(DefaultAppDelegate {})),
            themes: ThemeEngine::new(),
            windows: WindowManager::new()
        });

        let app_ptr: *const App = &*app;
        app.configure_bridge(app_ptr);
        app
    }

    /// Handles providing the app pointer to the inner bridge.
    pub(crate) fn configure_bridge(&self, ptr: *const App) {
        let mut bridge = self.bridge.lock().unwrap();
        *bridge = Some(PlatformAppBridge::new(ptr));
    }

    /// Convenience method for registering one-off styles. Typically, you would want 
    /// to store your stylesheets as separate files, to enable hot-reloading - but it's
    /// conceivable that you might want to just have them in your app, too, and this enables
    /// that use case.
    pub fn register_styles(&self, theme_key: &str, stylesheet: StyleSheet) {
        self.themes.register_styles(theme_key, stylesheet);
    }

    /// Runs the app instance, by setting the necessary delegate and forwarding the run call
    /// to the inner backing application. This is a blocking operation; if you run this, you
    /// will want to begin your app (for real) in `AppDelegate::did_finish_launching()`.
    pub fn run<S: 'static + AppDelegate>(&self, state: S) {
        {
            let mut delegate = self.delegate.lock().unwrap();
            *delegate = Box::new(state);
        }

        let lock = self.bridge.lock().unwrap();
        if let Some(bridge) = &*lock {
            bridge.run();
        }
    }
}

/// Implementing `AppDelegate` for `App` serves two purposes - for one, we're able to
/// separate the inner implementaton from the abstract one by referring to a trait type, avoiding
/// a cyclical dependency... and two, it allows us to react to these events on the App layer for
/// our own purposes, while still forwarding them on to the delegate.
impl AppDelegate for App {
    /// Called when the application will finish launching.
    fn will_finish_launching(&mut self) {
        let mut delegate = self.delegate.lock().unwrap();
        delegate.will_finish_launching();
    }
    
    /// Called when the application did finish launching.
    fn did_finish_launching(&mut self) { 
        let mut delegate = self.delegate.lock().unwrap();
        delegate.did_finish_launching();
    }

    /// Called when the application will become active. We can use this, for instance, 
    /// to resume rendering cycles and so on. 
    fn will_become_active(&mut self) {
        let mut delegate = self.delegate.lock().unwrap();
        delegate.will_become_active();
    }

    /// Called when the application did become active. We can use this, for instance, 
    /// to resume rendering cycles and so on.
    fn did_become_active(&mut self) {
        let mut delegate = self.delegate.lock().unwrap();
        delegate.did_become_active();
    }

    /// Called when the application will resigned active. We can use this, for instance, 
    /// to pause rendering cycles and so on.
    fn will_resign_active(&mut self) {
        let mut delegate = self.delegate.lock().unwrap();
        delegate.will_resign_active();
    }

    /// Called when the application has resigned active. We can use this, for instance, 
    /// to pause rendering cycles and so on.
    fn did_resign_active(&mut self) {
        let mut delegate = self.delegate.lock().unwrap();
        delegate.did_resign_active();
    }

    /// Called when the application should terminate - we can use it
    /// to avoid termination if Alchemy needs more time for something,
    /// for whatever reason.
    fn should_terminate(&self) -> bool {
        let delegate = self.delegate.lock().unwrap();
        delegate.should_terminate()
    }

    /// Called when the application is about to terminate.
    fn will_terminate(&mut self) {
        let mut delegate = self.delegate.lock().unwrap();
        delegate.will_terminate();
    }

    /// This is a private method, and you should not attempt to call it or
    /// rely on it. It exists to enable easy loopback of Window-level events
    /// on some platforms.
    fn _window_will_close(&self, window_id: usize) {
        self.windows.will_close(window_id);
    }
}
