//! A wrapper for `NSApplication` on macOS. If you opt in to the `cocoa` feature on
//! Alchemy, this will loop system-level application events back to your `AppDelegate`.

use std::sync::{Once};

use cocoa::base::{id, nil};
use cocoa::appkit::{NSApplication, NSRunningApplication};

use objc_id::Id;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{msg_send, class, sel, sel_impl};

use alchemy_lifecycle::traits::AppDelegate;

static ALCHEMY_APP_PTR: &str = "alchemyParentAppPtr";

/// A wrapper for `NSApplication`. It holds (retains) pointers for the Objective-C runtime,
/// which is where our application instance lives. It also injects an `NSObject` subclass,
/// which acts as the Delegate, looping back into our Alchemy shared application.
pub struct App {
    pub inner: Id<Object>,
    pub delegate: Id<Object>
}

impl App {
    /// Creates an NSAutoReleasePool, configures various NSApplication properties (e.g, activation
    /// policies), injects an `NSObject` delegate wrapper, and retains everything on the
    /// Objective-C side of things.
    pub fn new<T: AppDelegate>(parent_app_ptr: *const T) -> Self {
        let inner = unsafe {
            let _pool = cocoa::foundation::NSAutoreleasePool::new(nil);
            let app = cocoa::appkit::NSApp();
            app.setActivationPolicy_(cocoa::appkit::NSApplicationActivationPolicyRegular);
            Id::from_ptr(app)
        };

        let delegate = unsafe {
            let delegate_class = register_app_delegate_class::<T>();
            let delegate: id = msg_send![delegate_class, new];
            (&mut *delegate).set_ivar(ALCHEMY_APP_PTR, parent_app_ptr as usize);
            msg_send![&*inner, setDelegate:delegate];
            Id::from_ptr(delegate)
        };

        App {
            delegate: delegate,
            inner: inner
        }
    }

    /// Kicks off the NSRunLoop for the NSApplication instance. This blocks when called.
    pub fn run(&self) {
        unsafe {
            let current_app = cocoa::appkit::NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(cocoa::appkit::NSApplicationActivateIgnoringOtherApps);
            let shared_app: id = msg_send![class!(NSApplication), sharedApplication];
            msg_send![shared_app, run];
        }
    }
}

/// Fires when the Application Delegate receives a `applicationWillFinishLaunching` notification.
extern fn will_finish_launching<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    unsafe {
        let app_ptr: usize = *this.get_ivar(ALCHEMY_APP_PTR);
        let app = app_ptr as *mut T;
        (*app).will_finish_launching();
    };
}

/// Fires when the Application Delegate receives a `applicationDidFinishLaunching` notification.
extern fn did_finish_launching<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    unsafe {
        let app_ptr: usize = *this.get_ivar(ALCHEMY_APP_PTR);
        let app = app_ptr as *mut T;
        (*app).did_finish_launching();
    };
}

/// Fires when the Application Delegate receives a `applicationWillBecomeActive` notification.
extern fn will_become_active<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    unsafe {
        let app_ptr: usize = *this.get_ivar(ALCHEMY_APP_PTR);
        let app = app_ptr as *mut T;
        (*app).will_become_active();
    };
}

/// Fires when the Application Delegate receives a `applicationDidBecomeActive` notification.
extern fn did_become_active<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    unsafe {
        let app_ptr: usize = *this.get_ivar(ALCHEMY_APP_PTR);
        let app = app_ptr as *mut T;
        (*app).did_become_active();
    };
}

/// Fires when the Application Delegate receives a `applicationWillResignActive` notification.
extern fn will_resign_active<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    unsafe {
        let app_ptr: usize = *this.get_ivar(ALCHEMY_APP_PTR);
        let app = app_ptr as *mut T;
        (*app).will_resign_active();
    };
}

/// Fires when the Application Delegate receives a `applicationDidResignActive` notification.
extern fn did_resign_active<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    unsafe {
        let app_ptr: usize = *this.get_ivar(ALCHEMY_APP_PTR);
        let app = app_ptr as *mut T;
        (*app).did_resign_active();
    };
}

/// Fires when the Application Delegate receives a `applicationWillTerminate` notification.
extern fn will_terminate<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    unsafe {
        let app_ptr: usize = *this.get_ivar(ALCHEMY_APP_PTR);
        let app = app_ptr as *mut T;
        (*app).will_terminate();
    };
}

/// Registers an `NSObject` application delegate, and configures it for the various callbacks and
/// pointers we need to have.
fn register_app_delegate_class<T: AppDelegate>() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new("AlchemyAppDelegate", superclass).unwrap();

        decl.add_ivar::<usize>(ALCHEMY_APP_PTR);

        // Add callback methods
        decl.add_method(sel!(applicationWillFinishLaunching:), will_finish_launching::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(applicationDidFinishLaunching:), did_finish_launching::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(applicationWillBecomeActive:), will_become_active::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(applicationDidBecomeActive:), did_become_active::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(applicationWillResignActive:), will_resign_active::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(applicationDidResignActive:), did_resign_active::<T> as extern fn(&Object, _, _));
        decl.add_method(sel!(applicationWillTerminate:), will_terminate::<T> as extern fn(&Object, _, _));

        DELEGATE_CLASS = decl.register();
    });

    unsafe {
        DELEGATE_CLASS
    }
}
