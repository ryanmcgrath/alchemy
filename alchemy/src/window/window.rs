//! Implements the Window API. It attempts to provide a nice, common interface across
//! per-platform Window APIs.

use std::sync::{Arc, Mutex, RwLock};

use alchemy_lifecycle::traits::{Component, WindowDelegate};
use alchemy_lifecycle::rsx::{Props, RSX};

use alchemy_styles::Stretch;
use alchemy_styles::number::Number;
use alchemy_styles::geometry::Size;
use alchemy_styles::styles::{Style, Dimension};

use crate::{App, SHARED_APP};
use crate::components::View;
use crate::reconciler::{diff_and_patch_tree, walk_and_apply_styles};

#[cfg(feature = "cocoa")]
use alchemy_cocoa::window::{Window as PlatformWindowBridge};

/// Utility function for creating a root_node.
fn create_root_node(instance: Option<Arc<RwLock<Component>>>, layout_manager: &mut Stretch) -> RSX {
    let mut props = Props::default();
    props.styles = "root".into();
    
    let mut root_node = RSX::node("root", || Arc::new(RwLock::new(View::default())), props);
    
    if let RSX::VirtualNode(root) = &mut root_node {
        root.layout_node = match instance.is_some() {
            true => {
                let mut style = Style::default();
                style.size = Size {
                    width: Dimension::Points(600.),
                    height: Dimension::Points(600.)
                };
                
                match layout_manager.new_node(style, vec![]) {
                    Ok(node) => Some(node),
                    Err(e) => { None }
                }
            },

            false => None
        };
        
        root.instance = instance;
    }
    
    root_node
}

/// AppWindow contains the inner details of a Window. It's guarded by a Mutex on `Window`,
/// and you shouldn't create this yourself, but it's documented here so you can understand what
/// it holds.
pub struct AppWindow {
    pub id: usize,
    pub title: String,
    pub bridge: PlatformWindowBridge,
    pub delegate: Box<WindowDelegate>,
    pub root_node: RSX,
    pub layout: Stretch
}

impl AppWindow {
    /// Re-renders a window. This method calls `render()` on the `WindowDelegate`, patches it into
    /// the root tree node, and then diffs the old (current) tree against a new tree by walking it
    /// and determining what needs to be changed. This also calculates and applies layout and
    /// styling.
    ///
    /// This method is called on the `show` event, and in rare cases can be useful to call
    /// directly.
    pub fn render(&mut self) {
        let mut new_root_node = create_root_node(None, &mut self.layout);

        // For API reasons, we'll call the render for this Window, and then patch it into a new
        // root node for the tree diff/patch comparison. For this we only need to go one level
        // deep, the recursion in the next step will handle the rest.
        match self.delegate.render() {
            Ok(opt) => match opt {
                RSX::VirtualNode(mut child) => {
                    if let RSX::VirtualNode(root) = &mut new_root_node {
                        if child.tag == "Fragment" {
                            root.children.append(&mut child.children);
                        } else {
                            root.children.push(RSX::VirtualNode(child));
                        }
                    }
                },

                // If it's an RSX::None, or a RSX::VirtualText, we do nothing, as... one
                // requires nothing, and one isn't supported unless it's inside a <Text> tag.
                _ => {}
            },

            Err(e) => { eprintln!("Error rendering window! {}", e); }
        }

        // Taking ownership of the tree makes parts of this so much easier, so let's swap
        // them out for the moment. We're going to discard the old one anyway.
        let mut old_root_node = RSX::None;
        std::mem::swap(&mut old_root_node, &mut self.root_node);

        self.root_node = match diff_and_patch_tree(old_root_node, new_root_node, &mut self.layout, 0) {
            Ok(node) => node,
            Err(e) => { eprintln!("Error: {}", e); RSX::None }
        };

        self.configure_and_apply_styles();
    }

    /// Walks the tree again, purely concerning itself with calculating layout and applying styles.
    /// This in effect creates a two-pass layout system. In the future much of this may be made
    /// async, so relying on underlying behavior in here is considered... suspect.
    ///
    /// This method is called on window resize and show events.
    fn configure_and_apply_styles(&mut self) -> Result<(), Box<std::error::Error>> {
        let window_size = Size {
            width: Number::Defined(600.),
            height: Number::Defined(600.)
        };

        if let RSX::VirtualNode(root_node) = &mut self.root_node {
            if let Some(layout_node) = &root_node.layout_node {
                self.layout.compute_layout(*layout_node, window_size)?;
                walk_and_apply_styles(&root_node, &mut self.layout)?;
            }
        }

        Ok(())
    }

    /// Renders and calls through to the native platform window show method.
    pub fn show(&mut self) {
        self.render();
        self.bridge.show();
    }

    /// Calls through to the native platform window close method.
    pub fn close(&mut self) {
        self.bridge.close();
    }
}

/// Windows represented... well, a Window. When you create one, you get the Window back. When you
/// show one, a clone of the pointer is added to the window manager, and removed on close.
pub struct Window(pub(crate) Arc<Mutex<AppWindow>>);

impl Window {
    /// Creates a new window.
    pub fn new<S: 'static + WindowDelegate>(title: &str, dimensions: (f64, f64, f64, f64), delegate: S) -> Window {
        let window_id = SHARED_APP.windows.allocate_new_window_id();
        let mut layout = Stretch::new();
        let view = View::default();
        let shared_app_ptr: *const App = &**SHARED_APP;
        let bridge = PlatformWindowBridge::new(window_id, title, dimensions, &view, shared_app_ptr);
        
        Window(Arc::new(Mutex::new(AppWindow {
            id: window_id,
            title: title.into(),
            bridge: bridge,
            delegate: Box::new(delegate),
            root_node: create_root_node(Some(Arc::new(RwLock::new(view))), &mut layout),
            layout: layout
        })))
    }

    /// Renders a window. By default, a window renders nothing - make sure you implement `render()`
    /// on your `WindowDelegate`. Note that calling `.show()` implicitly calls this for you, so you
    /// rarely need to call this yourself.
    pub fn render(&self) {
        let mut window = self.0.lock().unwrap();
        window.render();
    }

    /// Registers this window with the window manager, renders it, and shows it.
    pub fn show(&self) {
        SHARED_APP.windows.add(self.0.clone());
        let mut window = self.0.lock().unwrap();
        window.show();
    }

    /// Hides a window. On some platforms, this is minimizing... on others, like macOS, it's
    /// actually hiding. On mobile, this shouldn't do anything.
    pub fn hide(&self) {

    }

    /// Closes the window, unregistering it from the window manager in the process and ensuring the
    /// necessary delegate method(s) are fired.
    pub fn close(&self) {
        let window_id = self.0.lock().unwrap().id; 
        SHARED_APP.windows.will_close(window_id);
        let mut window = self.0.lock().unwrap();
        window.close();
    }
}

impl Clone for Window {
    /// Clones a `Window` by cloning the inner `AppWindow`.
    fn clone(&self) -> Window {
        Window(self.0.clone())
    }
}

impl Drop for Window {
    /// When a `Window` is dropped, we want to ensure that it's closed, so we'll silently call
    /// `.close()` to be safe.
    fn drop(&mut self) {
        self.close();
    }
}
