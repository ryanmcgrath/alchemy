//! Implements a conversion method for taking an `alchemy::Color` and turning it into
//! an `NSColor`.

use objc_id::Id;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use core_graphics::base::CGFloat;

use alchemy_styles::color::Color;

pub trait IntoNSColor {
    fn into_nscolor(&self) -> Id<Object>;
}

impl IntoNSColor for Color {
    /// This creates an NSColor, retains it, and returns it. Dropping this value will
    /// call `release` on the Objective-C side.
    fn into_nscolor(&self) -> Id<Object> {
        let red = self.red as CGFloat / 255.0;
        let green = self.green as CGFloat / 255.0;
        let blue = self.blue as CGFloat / 255.0;
        let alpha = self.alpha as CGFloat / 255.0;
       
        unsafe {
            Id::from_ptr(msg_send![class!(NSColor), colorWithRed:red green:green blue:blue alpha:alpha])
        }
    }
}

