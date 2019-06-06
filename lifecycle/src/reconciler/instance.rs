//! Internal struct used for tracking component instances and their 
//! associated metadata (layout, appearance, etc).

use alchemy_styles::{Appearance, StylesList};
use alchemy_styles::stretch::node::{Node as LayoutNode};

use crate::traits::Component;

pub(crate) struct Instance {
    pub(crate) tag: &'static str,
    pub(crate) style_keys: StylesList,
    pub(crate) component: Box<Component + 'static>,
    pub(crate) appearance: Appearance,
    pub(crate) layout: Option<LayoutNode>
}
