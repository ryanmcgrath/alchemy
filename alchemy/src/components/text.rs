//! Handles hoisting per-platform specific Text components.
//! Each platform needs the freedom to do some specific things,
//! hence why they're all (somewhat annoyingly, but lovingly) re-implemented 
//! as bridges.

use std::sync::{Mutex};

use alchemy_styles::styles::{Appearance, Layout};

use alchemy_lifecycle::ComponentKey;
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
pub struct Text {
    text: String,
    bridge: Mutex<PlatformTextBridge>
}

impl Text {
    // This is very naive for now, but it's fine - we probably
    // want to do some fun stuff here later with stylized text
    // rendering anyway.
    fn compare_and_update_text(&mut self, props: &Props) {
        let text = props.children.iter().map(|child| match child {
            RSX::VirtualText(s) => s.0.clone(),
            _ => String::new()
        }).collect::<String>();
        
        if self.text != text {
            let mut bridge = self.bridge.lock().unwrap();
            bridge.set_text(&text);
            self.text = text;
        }
    }
}

impl Component for Text {
    fn constructor(_: ComponentKey) -> Text {
        Text {
            text: "".into(),
            bridge: Mutex::new(PlatformTextBridge::new())
        }
    }

    fn has_native_backing_node(&self) -> bool { true }
    
    fn borrow_native_backing_node(&self) -> Option<PlatformSpecificNodeType> {
        let bridge = self.bridge.lock().unwrap();
        Some(bridge.borrow_native_backing_node())
    }

    // Shouldn't be allowed to have child <Text> elements... or, should it?
    // Panic might not be right here, but eh, should probably do something.
    fn append_child_component(&self, _component: &Component) {}

    fn apply_styles(&self, appearance: &Appearance, layout: &Layout) {
        let mut bridge = self.bridge.lock().unwrap();
        bridge.apply_styles(appearance, layout);
    }

    fn component_did_mount(&mut self, props: &Props) {
        self.compare_and_update_text(props);
    }

    fn render(&self, _props: &Props) -> Result<RSX, Error> {
        Ok(RSX::None)
    }
}

