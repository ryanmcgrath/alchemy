//! This module implements the Application structure and associated 
//! lifecycle methods.
//!
//! This ensures that you can respond to application lifecycles, and so
//! routing things around works correctly.

use std::cell::RefCell;

use alchemy_lifecycle::traits::AppDelegate;

#[cfg(feature = "cocoa")]
pub use alchemy_cocoa::app::{App as PlatformAppBridge};

#[cfg(feature = "gtkrs")]
pub use alchemy_gtkrs::app::{App as PlatformAppBridge};

/// The Application structure itself. It holds a Mutex'd platform bridge, to
/// handle communicating with the platform-specific app instance, along with a
/// delegate to forward events to.
pub struct App {
    pub bridge: Option<RefCell<PlatformAppBridge>>,
    pub delegate: RefCell<Box<AppDelegate>>
}

impl App {
    /// Creates a new app, allocated on the heap. Provides a pointer to
    /// said allocated instance so that the platform-specific app instances
    /// can loop events back around.
    pub fn new<S: AppDelegate + 'static>(state: S) -> Box<App> {
        let mut app = Box::new(App {
            bridge: None,
            delegate: RefCell::new(Box::new(state))
        });

        let app_ptr: *const App = &*app;
        app.bridge = Some(RefCell::new(PlatformAppBridge::new(app_ptr)));

        app
    }

    /// Runs the app instance, by setting the necessary delegate and forwarding the run call
    /// to the inner backing application. This is a blocking operation; if you run this, you
    /// will want to begin your app (for real) in `AppDelegate::did_finish_launching()`.
    pub fn run(&self) {
        if let Some(bridge) = &self.bridge {
            bridge.borrow_mut().run();
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
        let mut delegate = self.delegate.borrow_mut();
        delegate.will_finish_launching();
    }
    
    /// Called when the application did finish launching.
    fn did_finish_launching(&mut self) { 
        let mut delegate = self.delegate.borrow_mut();
        delegate.did_finish_launching();
    }

    /// Called when the application will become active. We can use this, for instance, 
    /// to resume rendering cycles and so on. 
    fn will_become_active(&mut self) {
        let mut delegate = self.delegate.borrow_mut();
        delegate.will_become_active();
    }

    /// Called when the application did become active. We can use this, for instance, 
    /// to resume rendering cycles and so on.
    fn did_become_active(&mut self) {
        let mut delegate = self.delegate.borrow_mut();
        delegate.did_become_active();
    }

    /// Called when the application will resigned active. We can use this, for instance, 
    /// to pause rendering cycles and so on.
    fn will_resign_active(&mut self) {
        let mut delegate = self.delegate.borrow_mut();
        delegate.will_resign_active();
    }

    /// Called when the application has resigned active. We can use this, for instance, 
    /// to pause rendering cycles and so on.
    fn did_resign_active(&mut self) {
        let mut delegate = self.delegate.borrow_mut();
        delegate.did_resign_active();
    }

    /// Called when the application should terminate - we can use it
    /// to avoid termination if Alchemy needs more time for something,
    /// for whatever reason.
    fn should_terminate(&self) -> bool {
        let delegate = self.delegate.borrow_mut();
        delegate.should_terminate()
    }

    /// Called when the application is about to terminate.
    fn will_terminate(&mut self) {
        let mut delegate = self.delegate.borrow_mut();
        delegate.will_terminate();
    }
}
