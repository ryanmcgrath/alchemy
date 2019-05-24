//! A valid CSS class.
//!
//! A CSS class is a non-empty string that starts with an alphanumeric character
//! and is followed by any number of alphanumeric characters and the
//! `_`, `-` and `.` characters.

use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;
use std::str::FromStr;

/// A valid CSS class.
///
/// A CSS class is a non-empty string that starts with an alphanumeric character
/// and is followed by any number of alphanumeric characters and the
/// `_`, `-` and `.` characters.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StyleKey(String);

impl StyleKey {
    /// Construct a new styles list from a string.
    ///
    /// Returns `Err` if the provided string is invalid.
    pub fn try_new<S: Into<String>>(id: S) -> Result<Self, &'static str> {
        let id = id.into();
        {
            let mut chars = id.chars();
            match chars.next() {
                None => return Err("style keys cannot be empty"),
                Some(c) if !c.is_alphabetic() => {
                    return Err("style keys must start with an alphabetic character")
                }
                _ => (),
            }
            for c in chars {
                if !c.is_alphanumeric() && c != '-' {
                    return Err(
                        "style keys can only contain alphanumerics (dash included)",
                    );
                }
            }
        }
        Ok(StyleKey(id))
    }

    /// Construct a new class name from a string.
    ///
    /// Panics if the provided string is invalid.
    pub fn new<S: Into<String>>(id: S) -> Self {
        let id = id.into();
        Self::try_new(id.clone()).unwrap_or_else(|err| {
            panic!(
                "alchemy::dom::types::StyleKey: {:?} is not a valid class name: {}",
                id, err
            )
        })
    }
}

impl FromStr for StyleKey {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        StyleKey::try_new(s)
    }
}

impl<'a> From<&'a str> for StyleKey {
    fn from(str: &'a str) -> Self {
        StyleKey::from_str(str).unwrap()
    }
}

impl Display for StyleKey {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Display::fmt(&self.0, f)
    }
}

impl Deref for StyleKey {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
