//! components/label.rs
//!
//! Implements a Label Component struct. Used for TextNode
//! behind the scenes on most platforms.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 03/29/2019

use crate::prelude::RSX;
use crate::components::Component;
use crate::dom::elements::FlowContent;

#[derive(RSX, Debug, Default)]
pub struct Text {}

#[cfg(target_os = "macos")]
impl Component for Text {
    fn create_native_backing_node(&self) -> cocoa::base::id {
        use objc::{msg_send, sel, sel_impl};
        use cocoa::foundation::{NSRect, NSPoint, NSSize};
        use cocoa::base::id;
        use crate::components::macos::objc_classes::label;

        let view: cocoa::base::id;

        unsafe {
            let rect_zero = NSRect::new(NSPoint::new(0., 0.), NSSize::new(0., 0.));
            let alloc: id = msg_send![label::register_class(), alloc];
            view = msg_send![alloc, initWithFrame:rect_zero];
        }

        view
    }
}
