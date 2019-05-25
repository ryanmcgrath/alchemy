//! Handles hoisting per-platform specific Text components.
//! Each platform needs the freedom to do some specific things,
//! hence why they're all (somewhat annoyingly, but lovingly) re-implemented 
//! as bridges.

use std::sync::{Mutex};

use alchemy_styles::styles::{Layout, Style};

use alchemy_lifecycle::error::Error;
use alchemy_lifecycle::rsx::{Props, RSX};
use alchemy_lifecycle::traits::{Component, PlatformSpecificNodeType};

#[cfg(feature = "cocoa")]
use alchemy_cocoa::text::{Text as PlatformTextBridge};

/// Text rendering is a complicated mess, and being able to defer to the
/// backing platform for this is amazing. This is a very common Component.
///
/// Views accept styles and event callbacks as props. For example:
///
/// ```
/// <Text styles=["styleKey1", "styleKey2"] />
/// ```
pub struct Text(Mutex<PlatformTextBridge>);

impl Default for Text {
    fn default() -> Text {
        Text(Mutex::new(PlatformTextBridge::new()))
    }
}

impl Component for Text {
    fn has_native_backing_node(&self) -> bool { true }
    
    fn borrow_native_backing_node(&self) -> Option<PlatformSpecificNodeType> {
        let bridge = self.0.lock().unwrap();
        Some(bridge.borrow_native_backing_node())
    }

    // Shouldn't be allowed to have child <Text> elements... or, should it?
    // Panic might not be right here, but eh, should probably do something.
    fn append_child_component(&self, component: &Component) {}

    fn apply_styles(&self, layout: &Layout, style: &Style) {
        let mut bridge = self.0.lock().unwrap();
        bridge.apply_styles(layout, style);
    }

    fn component_did_mount(&mut self, props: &Props) {
        let mut bridge = self.0.lock().unwrap();
        bridge.set_text("LOL");
    }

    fn render(&self, props: &Props) -> Result<RSX, Error> {
        Ok(RSX::None)
    }
}

