//! Implements an `NSWindow` wrapper for MacOS, backed by
//! Cocoa and associated widgets. This also handles looping back
//! lifecycle events, such as window resizing or close events.

use std::sync::{Once, ONCE_INIT};

use cocoa::base::{id, nil, YES, NO};
use cocoa::appkit::{NSWindow, NSWindowStyleMask, NSBackingStoreType};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSString, NSAutoreleasePool};

use objc_id::ShareId;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{msg_send, sel, sel_impl};

use alchemy_lifecycle::traits::{AppDelegate, Component};
use alchemy_styles::Appearance;

static APP_PTR: &str = "alchemyAppPtr";
static WINDOW_MANAGER_ID: &str = "alchemyWindowManagerID";

/// A wrapper for `NSWindow`. Holds (retains) pointers for the Objective-C runtime 
/// where our `NSWindow` and associated delegate live.
pub struct Window {
    pub inner: ShareId<Object>,
    pub delegate: ShareId<Object>
}

impl Window {
    /// Creates a new `NSWindow` instance, configures it appropriately (e.g, titlebar appearance),
    /// injects an `NSObject` delegate wrapper, and retains the necessary Objective-C runtime
    /// pointers.
    pub fn new<T: AppDelegate>(window_id: usize, content_view: &Component, app_ptr: *const T) -> Window {
        let dimensions = NSRect::new(NSPoint::new(0., 0.), NSSize::new(0., 0.));

        let style = NSWindowStyleMask::NSResizableWindowMask |
            NSWindowStyleMask::NSUnifiedTitleAndToolbarWindowMask | NSWindowStyleMask::NSMiniaturizableWindowMask |
            NSWindowStyleMask::NSClosableWindowMask | NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSFullSizeContentViewWindowMask;

        let inner = unsafe {
            let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                dimensions, 
                style,
                NSBackingStoreType::NSBackingStoreBuffered,
                NO
            ).autorelease();

            msg_send![window, setTitlebarAppearsTransparent:YES];
            //msg_send![window, setTitleVisibility:1];

            // This is very important! NSWindow is an old class and has some behavior that we need
            // to disable, like... this. If we don't set this, we'll segfault entirely because the
            // Objective-C runtime gets out of sync.
            msg_send![window, setReleasedWhenClosed:NO];
            
            if let Some(view_ptr) = content_view.borrow_native_backing_node() {
                msg_send![window, setContentView:view_ptr];
            }

            ShareId::from_ptr(window)
        };
        
        let delegate = unsafe {
            let delegate_class = register_window_class::<T>();
            let delegate: id = msg_send![delegate_class, new];
            (&mut *delegate).set_ivar(APP_PTR, app_ptr as usize);
            (&mut *delegate).set_ivar(WINDOW_MANAGER_ID, window_id);
            msg_send![inner, setDelegate:delegate];
            ShareId::from_ptr(delegate)
        };

        Window {
            inner: inner,
            delegate: delegate
        }
    }

    pub fn set_title(&mut self, title: &str) {
        unsafe {
            let title = NSString::alloc(nil).init_str(title);
            msg_send![&*self.inner, setTitle:title];
        }
    }

    pub fn set_dimensions(&mut self, x: f64, y: f64, width: f64, height: f64) {
        unsafe {
            let dimensions = NSRect::new(
                NSPoint::new(x.into(), y.into()),
                NSSize::new(width.into(), height.into())
            );

            msg_send![&*self.inner, setFrame:dimensions display:YES];
        }
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
        unsafe {
            msg_send![&*self.inner, makeKeyAndOrderFront:nil];
        }
    }

    /// On macOS, calling `close()` is equivalent to calling... well, `close`. It closes the
    /// window.
    ///
    /// I dunno what else to say here, lol.
    ///
    /// You should never be calling this yourself, mind you - Alchemy core handles this for you.
    pub fn close(&self) {
        unsafe {
            msg_send![&*self.inner, close];
        }
    }
}

impl Drop for Window {
    /// When a Window is dropped on the Rust side, we want to ensure that we break the delegate
    /// link on the Objective-C side. While this shouldn't actually be an issue, I'd rather be
    /// safer than sorry.
    fn drop(&mut self) {
        // This bridging link needs to be broken on Drop.
        unsafe { 
            msg_send![&*self.inner, setDelegate:nil];
        }
    }
}

/// Called when a Window receives a `windowWillClose:` event. Loops back to the shared
/// Alchemy app instance, so that our window manager can act appropriately.
extern fn will_close<T: AppDelegate>(this: &Object, _: Sel, _: id) {
    unsafe {
        let app_ptr: usize = *this.get_ivar(APP_PTR);
        let window_id: usize = *this.get_ivar(WINDOW_MANAGER_ID);
        let app = app_ptr as *mut T;
        (*app)._window_will_close(window_id);
    };
}

/// Injects an `NSObject` delegate subclass, with some callback and pointer ivars for what we
/// need to do.
fn register_window_class<T: AppDelegate>() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new("alchemyWindowDelegateShim", superclass).unwrap();

        decl.add_ivar::<usize>(APP_PTR);
        decl.add_ivar::<usize>(WINDOW_MANAGER_ID);
        
        decl.add_method(sel!(windowWillClose:), will_close::<T> as extern fn(&Object, _, _));
        
        DELEGATE_CLASS = decl.register();
    });

    unsafe {
        DELEGATE_CLASS
    }
}
