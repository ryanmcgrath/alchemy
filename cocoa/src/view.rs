//! Implements a View Component struct. The most common
//! basic building block of any app. Wraps NSView on macOS.

use std::sync::{Once, ONCE_INIT};

use objc_id::{Id, ShareId};
use objc::{msg_send, sel, sel_impl};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};

use cocoa::base::{id, nil, YES};
use cocoa::foundation::{NSRect, NSPoint, NSSize};

use crate::color::IntoNSColor;

use alchemy_styles::{Appearance, Color, Layout};

use alchemy_lifecycle::traits::PlatformSpecificNodeType;

static ALCHEMY_DELEGATE: &str = "alchemyDelegate";
static BACKGROUND_COLOR: &str = "alchemyBackgroundColor";

/// A wrapper for `NSView`. This holds retained pointers for the Objective-C 
/// runtime - namely, the view itself, and associated things such as background
/// colors and so forth.
#[derive(Debug)]
pub struct View {
    inner_mut: Id<Object>,
    inner_share: ShareId<Object>,
    background_color: Id<Object>
}

impl View {
    /// Allocates a new `NSView` on the Objective-C side, ensuring that things like coordinate
    /// flipping occur (macOS still uses (0,0) as lower-left by default), and opting in to layer
    /// backed views for smoother scrolling.
    pub fn new() -> View {
        let (inner_mut, inner_share) = unsafe {
            let rect_zero = NSRect::new(NSPoint::new(0., 0.), NSSize::new(0., 0.));
            let alloc: id = msg_send![register_class(), alloc];
            let view: id = msg_send![alloc, initWithFrame:rect_zero];
            msg_send![view, setWantsLayer:YES];
            msg_send![view, setLayerContentsRedrawPolicy:1];
            let x = view.clone();
            (Id::from_ptr(view), ShareId::from_ptr(x))
        };

        View {
            inner_mut: inner_mut,
            inner_share: inner_share,
            background_color: Color::transparent().into_nscolor()
        }
    }

    /// Returns a pointer to the underlying Objective-C view. The pointer is not mutable; however,
    /// you can send messages to it (unsafely).
    pub fn borrow_native_backing_node(&self) -> PlatformSpecificNodeType {
        self.inner_share.clone()
    }

    /// Appends a child NSView (or subclassed type) to this view.
    pub fn append_child(&mut self, child: PlatformSpecificNodeType) {
        unsafe {
            msg_send![&*self.inner_mut, addSubview:child];
        }
    }

    /// Given a `&Style`, will set the frame, background color, borders and so forth. It then
    /// calls `setNeedsDisplay:YES` on the Objective-C side, so that Cocoa will re-render this
    /// view.
    pub fn apply_styles(&mut self, appearance: &Appearance, layout: &Layout) {
        unsafe {
            let rect = NSRect::new(
                NSPoint::new(layout.location.x.into(), layout.location.y.into()),
                NSSize::new(layout.size.width.into(), layout.size.height.into())
            );

            self.background_color = appearance.background_color.into_nscolor();
            self.inner_mut.set_ivar(BACKGROUND_COLOR, &*self.background_color); 
            
            msg_send![&*self.inner_mut, setFrame:rect];
            msg_send![&*self.inner_mut, setNeedsDisplay:YES];
        }
    }
}

/// This is used for some specific calls, where macOS NSView needs to be
/// forcefully dragged into the modern age (e.g, position coordinates from top left...).
extern fn enforce_normalcy(_: &Object, _: Sel) -> BOOL {
    return YES;
}

/// When an `NSView` has `updateLayer` called, it will get passed through here, at which point we
/// instruct the layer how it should render (e.g, background color).
extern fn update_layer(this: &Object, _: Sel) {
    unsafe {
        let background_color: id = *this.get_ivar(BACKGROUND_COLOR);
        if background_color != nil {
            let layer: id = msg_send![this, layer];
            let cg: id = msg_send![background_color, CGColor];
            msg_send![layer, setBackgroundColor:cg];
        }
    }
}

/// Registers an `NSView` subclass, and configures it to hold some ivars for various things we need
/// to store.
fn register_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSView").unwrap();
        let mut decl = ClassDecl::new("AlchemyView", superclass).unwrap();
        
        // Force NSView to render from the top-left, not bottom-left
        decl.add_method(sel!(isFlipped), enforce_normalcy as extern fn(&Object, _) -> BOOL);

        // Opt-in to AutoLayout
        //decl.add_method(sel!(requiresConstraintBasedLayout), enforce_normalcy as extern fn(&Object, _) -> BOOL);

        // Request optimized backing layers
        decl.add_method(sel!(updateLayer), update_layer as extern fn(&Object, _));
        decl.add_method(sel!(wantsUpdateLayer), enforce_normalcy as extern fn(&Object, _) -> BOOL);

        // Ensure mouse events and so on work
        //decl.add_method(sel!(acceptsFirstResponder), update_layer as extern fn(&Object, _));

        // A pointer back to our View, for forwarding mouse + etc events.
        // Note that NSView's don't really have a "delegate", I'm just using it here
        // for common terminology sake.
        decl.add_ivar::<usize>(ALCHEMY_DELEGATE);
        decl.add_ivar::<id>(BACKGROUND_COLOR);
       
        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
