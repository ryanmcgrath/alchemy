//! Implements an Error type. Currently we just alias this to
//! Box<Error>, because I'm not sure how this should really look. Consider
//! it an implementation detail hook that could change down the road.

/// A generic Error type that we use. It currently just aliases to `Box<std::error::Error>`,
/// but could change in the future.
pub type Error = Box<dyn std::error::Error>;
