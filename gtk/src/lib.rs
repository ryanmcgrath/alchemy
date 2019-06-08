//! This crate provides a GTK backend for Alchemy, the Rust GUI framework.
//! This means that, on GTK-based systems, you'll be using native views 
//! and other assorted controls. Where possible, it attempts to opt into 
//! smoother rendering paths.
//!
//! # License
//!
//! Copyright 2019 Ryan McGrath. See the license files included in the root repository
//! for more information, along with credit to applicable parties for who this project
//! would not have happened.
//!
//! # Code of Conduct
//!
//! Please note that this project is released with a [Contributor Code of
//! Conduct][coc]. By participating in this project you agree to abide by its terms.
//!
//! [coc]: https://www.contributor-covenant.org/version/1/4/code-of-conduct

pub mod app;
pub mod text;
pub mod view;
pub mod window;
