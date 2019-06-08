//! A wrapper for `Application` on GTK-based systems. If you opt in to the `gtkrs` feature on
//! Alchemy, this will loop system-level application events back to your `AppDelegate`.

use std::env;

use gio::{ApplicationFlags};
use gtk::{Application};
use gio::prelude::{ApplicationExt, ApplicationExtManual};

use alchemy_lifecycle::traits::AppDelegate;

/// A wrapper for `gtk::Application`. 
pub struct App {
    pub inner: Application
}

impl App {
    /// Creates a `gtk::Application` instance, and wires up appropriate lifecycle handlers to loop
    /// back to the `AppDelegate`.
    pub fn new<T: AppDelegate + 'static>(parent_app_ptr: *const T) -> Self {
        let inner = Application::new("lol.my.app", ApplicationFlags::FLAGS_NONE)
            .expect("Could not create GTK Application instance!");

        inner.connect_activate(move |app| {
            println!("ACTIVATED");
            let app = parent_app_ptr as *mut T;
            unsafe {
                (*app).did_finish_launching();
            }
            println!("HELLO");
        });

        println!("MADE");
        App {
            inner: inner
        }
    }

    /// Kicks off the Run Loop for the Application instance. This blocks when called.
    pub fn run(&self) {
        println!("RUNNING");
        self.inner.run(&env::args().collect::<Vec<_>>());
    }
}
