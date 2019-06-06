//! Handles hoisting per-platform specific Text components.
//! Each platform needs the freedom to do some specific things,
//! hence why they're all (somewhat annoyingly, but lovingly) re-implemented 
//! as bridges.

use std::sync::{Mutex};

use alchemy_styles::styles::{Appearance, Layout};

use alchemy_lifecycle::ComponentKey;
use alchemy_lifecycle::error::Error;
use alchemy_lifecycle::rsx::RSX;
use alchemy_lifecycle::traits::{Component, Props, PlatformSpecificNodeType};

#[cfg(feature = "cocoa")]
use alchemy_cocoa::text::{Text as PlatformTextBridge};

pub struct TextProps;

/// Text rendering is a complicated mess, and being able to defer to the
/// backing platform for this is amazing. This is a very common Component.
///
/// Views accept styles and event callbacks as props. For example:
///
/// ```
/// <Text styles=["styleKey1", "styleKey2"] />
/// ```
pub struct Text(Mutex<PlatformTextBridge>);

impl Text {
    pub fn default_props() -> TextProps { TextProps {} }
    // This is very naive for now, but it's fine - we probably
    // want to do some fun stuff here later with stylized text
    // rendering anyway.
    //fn compare_and_update_text(&mut self, props: &Props) {
        /*let text = props.*/
    //}
}

impl Props for Text {
    fn set_props(&mut self, _: &mut std::any::Any) {}
}

impl Component for Text {
    fn new(_: ComponentKey) -> Text {
        Text(Mutex::new(PlatformTextBridge::new()))
    }

    fn has_native_backing_node(&self) -> bool { true }
    
    fn borrow_native_backing_node(&self) -> Option<PlatformSpecificNodeType> {
        let bridge = self.0.lock().unwrap();
        Some(bridge.borrow_native_backing_node())
    }

    // Shouldn't be allowed to have child <Text> elements... or, should it?
    // Panic might not be right here, but eh, should probably do something.
    //fn append_child_component(&self, _component: &Component) {}

    fn apply_styles(&self, appearance: &Appearance, layout: &Layout) {
        let mut bridge = self.0.lock().unwrap();
        bridge.apply_styles(appearance, layout);
    }

    fn component_did_mount(&mut self) {
        let mut bridge = self.0.lock().unwrap();
        bridge.render();
    }

    // This one is a bit tricky, due to the way we have to do props + children in Rust.
    // Here, we set it as the new text on render(), and then ensure it gets rendered on
    // `component_did_update()` and `component_did_mount()`.
    fn render(&self, children: Vec<RSX>) -> Result<RSX, Error> {
        let text = children.iter().map(|child| match child {
            RSX::VirtualText(s) => s.0.to_owned(),
            _ => String::new()
        }).collect::<String>();
        
        let mut bridge = self.0.lock().unwrap();
        bridge.set_text(text);
        
        Ok(RSX::None)
    }
}
