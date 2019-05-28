//! Internal struct used for tracking component instances and their 
//! associated metadata (layout, appearance, etc).

use alchemy_styles::Appearance;
use alchemy_styles::stretch::node::{Node as LayoutNode};

use crate::rsx::Props;
use crate::traits::Component;

pub(crate) struct Instance {
    pub(crate) tag: &'static str,
    pub(crate) component: Box<Component>,
    pub(crate) props: Props,
    pub(crate) appearance: Appearance,
    pub(crate) layout: Option<LayoutNode>
}

impl Instance {
    pub(crate) fn new(
        tag: &'static str,
        component: Box<Component>,
        props: Props,
        layout: Option<LayoutNode>
    ) -> Instance {
        Instance {
            tag: tag,
            component: component,
            props: props,
            appearance: Appearance::default(),
            layout: layout
        }
    }
}
