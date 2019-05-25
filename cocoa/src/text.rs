//! This wraps NTextField on macOS, and configures it to act like a label
//! with standard behavior that most users would expect.

use std::sync::{Once, ONCE_INIT};

use objc_id::{Id, ShareId};
use objc::{msg_send, sel, sel_impl};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};

use cocoa::base::{id, nil, YES};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSString};

use crate::color::IntoNSColor;

use alchemy_styles::color::Color;
use alchemy_styles::styles::Style;
use alchemy_styles::result::Layout;

use alchemy_lifecycle::traits::PlatformSpecificNodeType;

static ALCHEMY_DELEGATE: &str = "alchemyDelegate";

/// A wrapper for `NSText`. This holds retained pointers for the Objective-C 
/// runtime - namely, the view itself, and associated things such as background
/// colors and so forth.
#[derive(Debug)]
pub struct Text {
    inner_mut: Id<Object>,
    inner_share: ShareId<Object>,
    background_color: Id<Object>,
    text_color: Id<Object>,
    //text: Id<Object>
}

impl Text {
    /// Allocates a new `NSTextField` on the Objective-C side, ensuring that things like coordinate
    /// flipping occur (macOS still uses (0,0) as lower-left by default), and opting in to layer
    /// backed views for smoother scrolling.
    pub fn new() -> Text {
        let (inner_mut, inner_share) = unsafe {
            let initial_string = NSString::alloc(nil).init_str("wut wut");
            let view: id = msg_send![register_class(), labelWithString:initial_string];
            msg_send![view, setSelectable:YES];
            msg_send![view, setDrawsBackground:YES];
            msg_send![view, setWantsLayer:YES];
            msg_send![view, setLayerContentsRedrawPolicy:1];
            let x = view.clone();
            (Id::from_ptr(view), ShareId::from_ptr(x)) //, Id::from_ptr(initial_string))
        };

        Text {
            inner_mut: inner_mut,
            inner_share: inner_share,
            background_color: Color::transparent().into_nscolor(),
            text_color: Color::transparent().into_nscolor(),
       //     text: s
        }
    }

    /// Returns a pointer to the underlying Objective-C view. The pointer is not mutable; however,
    /// you can send messages to it (unsafely).
    pub fn borrow_native_backing_node(&self) -> PlatformSpecificNodeType {
        self.inner_share.clone()
    }

    /// Appends a child NSText (or subclassed type) to this view.
    pub fn append_child(&mut self, child: PlatformSpecificNodeType) {
        unsafe {
            msg_send![&*self.inner_mut, addSubview:child];
        }
    }

    /// Given a `&Style`, will set the frame, background color, borders and so forth. It then
    /// calls `setNeedsDisplay:YES` on the Objective-C side, so that Cocoa will re-render this
    /// view.
    pub fn apply_styles(&mut self, layout: &Layout, style: &Style) {
        unsafe {
            let rect = NSRect::new(
                NSPoint::new(layout.location.x.into(), layout.location.y.into()),
                NSSize::new(layout.size.width.into(), layout.size.height.into())
            );

            self.background_color = style.background_color.into_nscolor();
            self.text_color = style.text_color.into_nscolor();
            
            msg_send![&*self.inner_mut, setFrame:rect];
            msg_send![&*self.inner_mut, setBackgroundColor:&*self.background_color];
            msg_send![&*self.inner_mut, setTextColor:&*self.text_color];
        }
    }

    pub fn set_text(&mut self, text: &str) {
        unsafe {
            let string_value = NSString::alloc(nil).init_str(text);
            msg_send![&*self.inner_mut, setStringValue:string_value];
        }
    }
}

/// This is used for some specific calls, where macOS NSText needs to be
/// forcefully dragged into the modern age (e.g, position coordinates from top left...).
extern fn enforce_normalcy(_: &Object, _: Sel) -> BOOL {
    return YES;
}

/// Registers an `NSText` subclass, and configures it to hold some ivars for various things we need
/// to store.
fn register_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSTextField").unwrap();
        let mut decl = ClassDecl::new("AlchemyTextField", superclass).unwrap();
        
        // Force NSText to render from the top-left, not bottom-left
        //decl.add_method(sel!(isFlipped), enforce_normalcy as extern fn(&Object, _) -> BOOL);

        // Request optimized backing layers
        //decl.add_method(sel!(updateLayer), update_layer as extern fn(&Object, _));
        //decl.add_method(sel!(wantsUpdateLayer), enforce_normalcy as extern fn(&Object, _) -> BOOL);

        // Ensure mouse events and so on work
        //decl.add_method(sel!(acceptsFirstResponder), update_layer as extern fn(&Object, _));

        // A pointer back to our Text, for forwarding mouse + etc events.
        // Note that NSText's don't really have a "delegate", I'm just using it here
        // for common terminology sake.
        decl.add_ivar::<usize>(ALCHEMY_DELEGATE);
       
        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
