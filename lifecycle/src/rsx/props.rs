//! Implements a Props struct that mostly acts as expected. For arbitrary primitive values,
//! it shadows a `serde_json::Value`.

use std::sync::{Arc, RwLock};
use serde_json::Value;
use std::collections::HashMap;

use alchemy_styles::StylesList;

use crate::rsx::RSX;
use crate::traits::{Component};

/// A value stored inside the `attributes` field on a `Props` instance.
/// It shadows `serde_json::Value`, but also allows for some other value
/// types common to Alchemy.
#[derive(Clone, Debug)]
pub enum AttributeType {
    Value(Value),
    //RSX(RSX)
    //EventHandler(Box<ComponentEventHandler>)
}

impl<'a> From<&'a str> for AttributeType {
    /// Converts a &str to a storable AttributeType.
    fn from(f: &str) -> Self {
        AttributeType::Value(Value::String(f.to_string()))
    }
}

/// Emulates props from React, in a sense. Common keys such as `children`, `key` and `styles` 
/// are extracted out for fast access, and everything else found gets put into the `attributes` 
/// HashMap.
#[derive(Clone, Debug, Default)]
pub struct Props {
    pub attributes: HashMap<&'static str, AttributeType>,
    pub children: Vec<RSX>,
    pub key: String,
    pub styles: StylesList
}

impl Props {
    pub fn new(key: String, styles: StylesList, attributes: HashMap<&'static str, AttributeType>) -> Props {
        Props {
            attributes: attributes,
            children: vec![],
            key: key,
            styles: styles
        }
    }

    /// Returns a Vec of RSX nodes, which are really just cloned pointers for the most part.
    pub fn children(&self) -> Vec<RSX> {
        self.children.clone()
    }

    /// Returns a Option<&AttributeType> from the `attributes` inner HashMap.
    pub fn get(&self, key: &str) -> Option<&AttributeType> {
        match key {
            "children" => { None },
            "key" => { None },
            "styles" => { None },
            _ => { None } //self.attributes.get(key) }
        }
    }
}
