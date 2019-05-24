//! This module implements basic core Components (View, Label, etc).
//! End-users are of course free to implement their own; the core
//! Components in this module should just be enough to build a
//! functioning app.

pub mod fragment;
pub mod view;
//pub mod text;

pub use fragment::Fragment;
pub use view::View;
//pub use text::*;
