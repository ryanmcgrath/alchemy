//! This wraps NTextField on macOS, and configures it to act like a label
//! with standard behavior that most users would expect.

use alchemy_styles::{Color, Layout, Appearance};

use alchemy_lifecycle::traits::PlatformSpecificNodeType;

static ALCHEMY_DELEGATE: &str = "alchemyDelegate";

/// A wrapper for `NSText`. This holds retained pointers for the Objective-C 
/// runtime - namely, the view itself, and associated things such as background
/// colors and so forth.
#[derive(Debug)]
pub struct Text {
}

impl Text {
    /// Allocates a new `NSTextField` on the Objective-C side, ensuring that things like coordinate
    /// flipping occur (macOS still uses (0,0) as lower-left by default), and opting in to layer
    /// backed views for smoother scrolling.
    pub fn new() -> Text {
        Text {
        }
    }

    /// Returns a pointer to the underlying Objective-C view. The pointer is not mutable; however,
    /// you can send messages to it (unsafely).
    pub fn borrow_native_backing_node(&self) -> PlatformSpecificNodeType {
        ()
    }

    /// Appends a child NSText (or subclassed type) to this view.
    pub fn append_child(&mut self, child: PlatformSpecificNodeType) {
    }

    /// Given a `&Style`, will set the frame, background color, borders and so forth. It then
    /// calls `setNeedsDisplay:YES` on the Objective-C side, so that Cocoa will re-render this
    /// view.
    pub fn apply_styles(&mut self, appearance: &Appearance, layout: &Layout) {
    }

    pub fn set_text(&mut self, text: String) {
    }

    pub fn render(&mut self) {
    }
}
