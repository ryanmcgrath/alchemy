//! A Fragment is for components that want to return or hoist multiple inner
//! child nodes. `impl IntoIterator` can't be used in trait returns right now,
//! and this API more or less matches what React presents, so I'm fine with it...
//! but as the language stabilizes even further I'd love to get rid of this and
//! just allow returning arbitrary iterators.

use alchemy_lifecycle::ComponentKey;
use alchemy_lifecycle::traits::{Component, Props};

pub struct FragmentProps;

/// Fragments are special - you can do something like the following in cases where you
/// want to render some views without requiring an intermediate view.
///
/// ```ignore
/// <Fragment>
///     <View />
///     <View />
///     <View />
/// </Fragment>
/// ```
#[derive(Default, Debug)]
pub struct Fragment;

impl Fragment {
    pub fn default_props() -> FragmentProps {
        FragmentProps {}
    }
}

impl Props for Fragment {
    fn set_props(&mut self, _: &mut dyn std::any::Any) {}
}

impl Component for Fragment {
    fn new(_: ComponentKey) -> Fragment {
        Fragment {}
    }
}
