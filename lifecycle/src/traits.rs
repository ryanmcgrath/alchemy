//! Traits that are used in Alchemy. Alchemy implements a React-based Component
//! lifecycle, coupled with a delegate pattern inspired by those found in AppKit/UIKit.

use std::sync::Arc;

use alchemy_styles::styles::{Layout, Style};

use crate::error::Error;
use crate::rsx::{RSX, Props};

/// A per-platform wrapped Pointer type, used for attaching views/widgets.
#[cfg(feature = "cocoa")]
pub type PlatformSpecificNodeType = objc_id::ShareId<objc::runtime::Object>;

/// A per-platform wrapped Pointer type, used for attaching views/widgets.
#[cfg(not(feature = "cocoa"))]
pub type PlatformSpecificNodeType = ();

/// Each platform tends to have their own startup routine, their own runloop, and so on.
/// Alchemy recognizes this and provides an `AppDelegate` that receives events at a system
/// level and allows the user to operate within the established framework per-system.
pub trait AppDelegate: Send + Sync {
    /// Fired when an Application is about to finish launching.
    fn will_finish_launching(&mut self) {}

    /// Fired when an Application has finished launching - this is a good place to, say, show your
    /// window.
    fn did_finish_launching(&mut self) {}

    /// Fired when an Application will become active.
    fn will_become_active(&mut self) {}

    /// Fired when an Application became active.
    fn did_become_active(&mut self) {}

    /// Fired when an Application will resign active. You can use this to, say, persist resources
    /// or state.
    fn will_resign_active(&mut self) {}

    /// Fired when an Application has resigned active.
    fn did_resign_active(&mut self) {} 

    /// Fired when an Application is going to terminate. You can use this to, say, instruct the
    /// system to "wait a minute, lemme finish".
    fn should_terminate(&self) -> bool { true }

    /// Fired when the Application has determined "no, you're done, stop the world".
    fn will_terminate(&mut self) {}

    /// A private trait method that you shouldn't call. This may change or disappear in later
    /// releases. Do not rely on this.
    fn _window_will_close(&self, _window_id: usize) {}
}

/// Each platform has their own `Window` API, which Alchemy attempts to pair down to one consistent
/// API. This also acts as the bootstrapping point for a `render` tree.
pub trait WindowDelegate: Send + Sync {
    /// Fired when this Window will close. You can use this to clean up or destroy resources,
    /// timers, and other things.
    fn will_close(&mut self) { }

    /// Called as the first step in the `render` tree. Every Window contains its own content view
    /// that is special, called the root. Widget trees are added to it as necessary, bootstrapped
    /// from here.
    fn render(&self) -> Result<RSX, Error> { Ok(RSX::None) }
}

pub trait State {}

/// The `Component` lifecycle, mostly inspired from React, with a few extra methods for views that
/// need to have a backing native layer. A good breakdown of the React Component lifecycle can be 
/// found [in this tweet](https://twitter.com/dan_abramov/status/981712092611989509?lang=en).
///
/// Alchemy does not currently implement Hooks, and at the moment has no plans to do so (the API
/// doesn't feel comfortable in Rust, in any way I tried). If you think you have an interesting
/// proposal for this, feel free to open an issue!
pub trait Component: Send + Sync {
    /// Indicates whether a Component instance carries a native backing node. If you return `true`
    /// from this, the reconciler will opt-in to the native backing layer. Returns `false` by
    /// default.
    fn has_native_backing_node(&self) -> bool { false }

    /// Returns a wrapped-per-platform pointer type that the backing framework tree can use.
    fn borrow_native_backing_node(&self) -> Option<PlatformSpecificNodeType> { None }

    /// If you implement a Native-backed component, you'll need to implement this. Given a
    /// `component`, you need to instruct the system how to append it to the tree at your point.
    fn append_child_component(&self, _component: &Arc<Component>) {}

    /// If you implement a Native-backed component, you'll need to implement this. Given a
    /// `component`, you need to instruct the system how to replace it in the tree at your point.
    fn replace_child_component(&self, _component: Arc<Component>) {}

    /// If you implement a Native-backed component, you'll need to implement this. Given a
    /// `component`, you need to instruct the system how to remove it from the tree at your point.
    fn remove_child_component(&self, _component: Arc<Component>) {}

    /// Given a computed `layout`, and an accompanying `Style` (which holds appearance-based
    /// styles, like colors), this method should transform them into appropriate calls to the
    /// backing native node.
    fn apply_styles(&self, _layout: &Layout, _style: &Style) {}

    /// Invoked right before calling the render method, both on the initial mount and on subsequent updates.
    /// It should return an object to update the state, or null to update nothing.
    /// This method exists for rare use cases where the state depends on changes in props over time.
    fn get_derived_state_from_props(&self, _props: Props) {}
    
    /// Invoked right before the most recently rendered output is committed to the backing layer tree.
    /// It enables your component to capture some information from the tree (e.g. scroll position) before it's 
    /// potentially changed. Any value returned by this lifecycle will be passed as a parameter 
    /// to component_did_update().
    /// 
    /// This use case is not common, but it may occur in UIs like a chat thread that need to handle scroll 
    /// position in a special way. A snapshot value (or None) should be returned.
    fn get_snapshot_before_update(&self, _props: Props) {}

    /// Invoked immediately after a component is mounted (inserted into the tree).
    /// If you need to load data from a remote endpoint, this is a good place to instantiate the network request.
    /// This method is also a good place to set up any subscriptions. If you do that, don’t forget to unsubscribe 
    /// in component_will_unmount().
    fn component_did_mount(&mut self, _props: &Props) {}

    /// Invoked immediately after updating occurs. This method is not called for the initial render.
    /// This is also a good place to do network requests as long as you compare the current props to previous props 
    /// (e.g. a network request may not be necessary if the props have not changed).
    fn component_did_update(&mut self, _props: &Props) {}

    /// Invoked immediately before a component is unmounted and destroyed. Perform any necessary cleanup in this 
    /// method, such as invalidating timers, canceling network requests, or cleaning up any subscriptions that 
    /// were created in component_did_mount().
    /// 
    /// You should not call set state in this method because the component will never be re-rendered. Once a 
    /// component instance is unmounted, it will never be mounted again.
    fn component_will_unmount(&mut self, _props: &Props) {}

    /// Invoked after an error has been thrown by a descendant component. Called during the "commit" phase, 
    /// so side-effects are permitted. It should be used for things like logging errors (e.g,
    /// Sentry).
    fn component_did_catch(&mut self, _props: &Props/* error: */) {}

    /// Use this to let Alchemy know if a component’s output is not affected by the current change in state 
    /// or props. The default behavior is to re-render on every state change, and in the vast majority of 
    /// cases you should rely on the default behavior.
    ///
    /// This is invoked before rendering when new props or state are being received. Defaults to true. This 
    /// method is not called for the initial render or when force_update() is used. This method only exists 
    /// as a performance optimization. Do not rely on it to “prevent” a rendering, as this can lead to bugs.
    fn should_component_update(&self) -> bool { true }

    /// The only required method for a `Component`. Should return a Result of RSX nodes, or an
    /// Error (in very rare cases, such as trying to get a key from a strange HashMap or
    /// something). 
    ///
    /// The render() function should be pure, meaning that it does not modify component state, it 
    /// returns the same result each time it’s invoked, and it does not directly interact with the 
    /// backing rendering framework.
    ///
    /// If you need to interact with the browser, perform your work in component_did_mount() or the other 
    /// lifecycle methods instead. Keeping `render()` pure makes components easier to think about.
    ///
    /// This method is not called if should_component_update() returns `false`.
    fn render(&self, _props: &Props) -> Result<RSX, Error> { Ok(RSX::None) }

    /// This lifecycle is invoked after an error has been thrown by a descendant component. It receives 
    /// the error that was thrown as a parameter and should return a value to update state.
    ///
    /// This is called during the "render" phase, so side-effects are not permitted. 
    /// For those use cases, use component_did_catch() instead.
    fn get_derived_state_from_error(&self, _error: ()) {}

    /// By default, when your component’s state or props change, your component will re-render. 
    /// If your `render()` method depends on some other data, you can tell Alchemy that the component 
    /// needs re-rendering by calling `force_update()`.
    ///
    /// Calling `force_update()` will cause `render()` to be called on the component, skipping 
    /// `should_component_update()`. This will trigger the normal lifecycle methods for child components, 
    /// including the `should_component_update()` method of each child. Alchemy will still only update the 
    /// backing widget tree if the markup changes.
    ///
    /// Normally, you should try to avoid all uses of `force_update()` and only read from `this.props` 
    /// and `this.state` in `render()`.
    fn force_update(&self) {}
}
