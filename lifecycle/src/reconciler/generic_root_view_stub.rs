//! This is a generic view used for avoiding a circular dependency issue.
//! You should never need to touch this.

use std::any::Any;

use crate::ComponentKey;
use crate::traits::{Component, Props};

#[derive(Default)]
pub struct GenericRootViewProps;

/// This is never actually created, and is here primarily to avoid a circular
/// depedency issue (we can't import the View from alchemy's core crate, since the core crate
/// depends on this crate).
pub struct GenericRootView;

impl GenericRootView {
    fn get_default_props() -> GenericRootViewProps {
        GenericRootViewProps {}
    }
}

impl Props for GenericRootView {
    fn set_props(&mut self, _: &mut Any) {}    
}

impl Component for GenericRootView {
    fn new(_: ComponentKey) -> GenericRootView {
        GenericRootView {}
    }
}
