//! Handles hoisting per-platform specific View components.
//! Each platform needs the freedom to do some specific things,
//! hence why they're all (somewhat annoyingly, but lovingly) re-implemented 
//! as bridges.

use std::sync::Mutex;

use alchemy_styles::{Appearance, Layout};

use alchemy_lifecycle::ComponentKey;
use alchemy_lifecycle::error::Error;
use alchemy_lifecycle::rsx::RSX;
use alchemy_lifecycle::traits::{Component, Props, PlatformSpecificNodeType};

use crate::components::Fragment;

#[cfg(feature = "cocoa")]
use alchemy_cocoa::view::{View as PlatformViewBridge};

pub struct ViewProps;

/// Views are the most basic piece of the API. If you want to display something, you'll
/// probably be reaching for a View first and foremost.
///
/// Views accept styles and event callbacks as props. For example:
///
/// ```
/// <View styles=["styleKey1", "styleKey2"] />
/// ```
pub struct View {
    bridge: Mutex<PlatformViewBridge>
}

impl Default for View {
    fn default() -> View {
        View {
            bridge: Mutex::new(PlatformViewBridge::new())
        }
    }
}

impl View {
    pub fn default_props() -> ViewProps {
        ViewProps {}
    }
}

impl Props for View {
    fn set_props(&mut self, _: &mut std::any::Any) {}
}

impl Component for View {
    fn new(_: ComponentKey) -> View {
        View::default()
    }

    fn has_native_backing_node(&self) -> bool { true }
    
    fn borrow_native_backing_node(&self) -> Option<PlatformSpecificNodeType> {
        let bridge = self.bridge.lock().unwrap();
        Some(bridge.borrow_native_backing_node())
    }

    fn append_child_node(&self, node: PlatformSpecificNodeType) {
        let mut bridge = self.bridge.lock().unwrap();
        bridge.append_child(node);
    }

    fn apply_styles(&self, appearance: &Appearance, layout: &Layout) {
        let mut bridge = self.bridge.lock().unwrap();
        bridge.apply_styles(appearance, layout);
    }

    fn render(&self, children: Vec<RSX>) -> Result<RSX, Error> {
        Ok(RSX::node("Fragment", "".into(), |key| {
            Box::new(<Fragment as Component>::new(key))
        }, Box::new(ViewProps {}), children))
    }
}
