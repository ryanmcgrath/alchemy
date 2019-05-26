//! Lifecycle aspects for Alchemy.
//!
//! What's a lifecycle? Well, it includes things like delegates (App+Window), 
//! where they act as hooks for the system to inform you of events. It includes 
//! things like `Component`s, which instruct your views how to exist.
//!
//! It also includes the `RSX` enum, which is what `render()` methods generally
//! return. It's common enough to multiple crates, and is intricately linked to the
//! `Component` lifecycle, so it'll live here.
//!
//! This crate also includes the diffing and patching system for the widget tree - 
//! it needs to live with the `Component` lifecycle to enable state updating.

pub use uuid::Uuid;

use alchemy_styles::lazy_static;

pub mod error;
pub mod rsx;
pub mod traits;

mod reconciler;
use reconciler::RenderEngine;

lazy_static! {
    pub static ref RENDER_ENGINE: RenderEngine = RenderEngine::new();
}
