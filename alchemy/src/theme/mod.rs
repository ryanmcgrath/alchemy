//! Implements a Theme loader, which scans a few places and loads any
//! CSS files that are necessary.

use std::fs;
use std::env;
use std::sync::RwLock;
use std::path::PathBuf;
use std::collections::HashMap;

use toml;
use serde::Deserialize;

use alchemy_lifecycle::rsx::StylesList;

pub use alchemy_styles::color;
pub use alchemy_styles::styles;
pub use styles::{Style, Styles};

pub mod stylesheet;
pub use stylesheet::StyleSheet;

static CONFIG_FILE_NAME: &str = "alchemy.toml";

#[derive(Debug, Deserialize)]
struct RawConfig<'d> {
    #[serde(borrow)]
    general: Option<General<'d>>,
}

#[derive(Debug, Deserialize)]
struct General<'a> {
    #[serde(borrow)]
    dirs: Option<Vec<&'a str>>
}

/// The `ThemeEngine` controls loading themes and registering associated
/// styles.
#[derive(Debug)]
pub struct ThemeEngine {
    pub dirs: Vec<PathBuf>,
    pub themes: RwLock<HashMap<String, StyleSheet>>
}

impl ThemeEngine {
    /// Creates a new 'ThemeEngine` instance.
    pub fn new() -> ThemeEngine {
        // This env var is set by Cargo... so if this code breaks, there's
        // bigger concerns, lol
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        let root = PathBuf::from(manifest_dir);
        let default_dirs = vec![root.join("themes")];
        
        let toml_contents = read_config_file();
        let raw: RawConfig<'_> = toml::from_str(&toml_contents).expect(&format!("Invalid TOML in {}!", CONFIG_FILE_NAME));

        let dirs = match raw.general {
            Some(General { dirs }) => (
                dirs.map_or(default_dirs, |v| {
                    v.into_iter().map(|dir| root.join(dir)).collect()
                })
            ),

            None => default_dirs
        };

        ThemeEngine { dirs, themes: RwLock::new(HashMap::new()) }
    }

    /// Registers a stylesheet (typically created by the `styles! {}` macro) for a given
    /// theme.
    pub fn register_styles(&self, key: &str, stylesheet: StyleSheet) {
        let mut themes = self.themes.write().unwrap();
        if !themes.contains_key(key) {
            themes.insert(key.to_string(), stylesheet);
            return;
        }

        // if let Some(existing_stylesheet) = self.themes.get_mut(key) {
        //    *existing_stylesheet.merge(stylesheet);
        //}
    }

    /// Given a theme key, style keys, and a style, configures the style for layout
    /// and appearance.
    pub fn configure_style_for_keys_in_theme(&self, theme: &str, keys: &StylesList, style: &mut Style) {
        let themes = self.themes.read().unwrap();

        match themes.get(theme) {
            Some(theme) => {
                for key in &keys.0 {
                    theme.apply_styles(key, style);
                }
            },

            None => {
                eprintln!("No styles for theme!");
            }
        }
    }

    /// The same logic as `configure_style_for_keys_in_theme`, but defaults to the default theme.
    pub fn configure_style_for_keys(&self, keys: &StylesList, style: &mut Style) {
        self.configure_style_for_keys_in_theme("default", keys, style)
    }
}

/// Utility method for reading a config file from the `CARGO_MANIFEST_DIR`. Hat tip to 
/// [askama](https://github.com/djc/askama) for this!
pub fn read_config_file() -> String {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = PathBuf::from(manifest_dir);
    let filename = root.join(CONFIG_FILE_NAME);

    if filename.exists() {
        fs::read_to_string(&filename)
            .expect(&format!("Unable to read {}", filename.to_str().unwrap()))
    } else {
        "".to_string()
    }
}
