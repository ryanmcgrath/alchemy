//! Handles hoisting per-platform specific View components.
//! Each platform needs the freedom to do some specific things,
//! hence why they're all (somewhat annoyingly, but lovingly) re-implemented 
//! as bridges.

use std::sync::Mutex;

use alchemy_styles::{Appearance, Layout, StylesList};

use alchemy_lifecycle::ComponentKey;
use alchemy_lifecycle::error::Error;
use alchemy_lifecycle::rsx::{Props, RSX};
use alchemy_lifecycle::traits::{Component, PlatformSpecificNodeType};

use crate::components::Fragment;

#[cfg(feature = "cocoa")]
use alchemy_cocoa::view::{View as PlatformViewBridge};

/// Views are the most basic piece of the API. If you want to display something, you'll
/// probably be reaching for a View first and foremost.
///
/// Views accept styles and event callbacks as props. For example:
///
/// ```
/// <View styles=["styleKey1", "styleKey2"] />
/// ```
pub struct View(Mutex<PlatformViewBridge>);

impl Default for View {
    fn default() -> View {
        View(Mutex::new(PlatformViewBridge::new()))
    }
}

impl Component for View {
    fn constructor(_key: ComponentKey) -> View {
        View(Mutex::new(PlatformViewBridge::new()))
    }

    fn has_native_backing_node(&self) -> bool { true }
    
    fn borrow_native_backing_node(&self) -> Option<PlatformSpecificNodeType> {
        let bridge = self.0.lock().unwrap();
        Some(bridge.borrow_native_backing_node())
    }

    fn append_child_component(&self, component: &Component) {
        if let Some(child) = component.borrow_native_backing_node() {
            let mut bridge = self.0.lock().unwrap();
            bridge.append_child(child);
        }
    }

    fn apply_styles(&self, appearance: &Appearance, layout: &Layout) {
        let mut bridge = self.0.lock().unwrap();
        bridge.apply_styles(appearance, layout);
    }

    fn render(&self, props: &Props) -> Result<RSX, Error> {
        Ok(RSX::node("Fragment", |key| Box::new(Fragment::constructor(key)), Props {
            attributes: std::collections::HashMap::new(),
            key: "".into(),
            styles: StylesList::new(),
            children: vec![]
        }, props.children.clone()))
    }
}
