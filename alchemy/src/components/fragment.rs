//! A Fragment is for components that want to return or hoist multiple inner
//! child nodes. `impl IntoIterator` can't be used in trait returns right now,
//! and this API more or less matches what React presents, so I'm fine with it...
//! but as the language stabilizes even further I'd love to get rid of this and 
//! just allow returning arbitrary iterators.

use alchemy_lifecycle::ComponentKey;
use alchemy_lifecycle::traits::Component;

/// Fragments are special - you can do something like the following in cases where you
/// want to render some views without requiring an intermediate view.
///
/// ```
/// <Fragment>
///     <View />
///     <View />
///     <View />
/// </Fragment>
/// ```
#[derive(Default, Debug)]
pub struct Fragment;

impl Component for Fragment {
    fn constructor(_key: ComponentKey) -> Fragment {
        Fragment { }
    }
}
