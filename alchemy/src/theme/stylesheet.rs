//! Implements a `StyleSheet`, which contains inner logic for
//! determining what styles should be applied to a given widget.

use std::collections::HashMap;
use alchemy_styles::styles::{Dimension, Rect, Size, Style, Styles};

/// A `StyleSheet` contains selectors and parsed `Styles` attributes.
/// It also has some logic to apply styles for n keys to a given `Style` node.
#[derive(Debug)]
pub struct StyleSheet(HashMap<&'static str, Vec<Styles>>);

impl StyleSheet {
    /// Creates a new `Stylesheet`.
    pub fn new(styles: HashMap<&'static str, Vec<Styles>>) -> Self {
        StyleSheet(styles)
    }

    pub fn apply_styles(&self, key: &str, style: &mut Style) {
        match self.0.get(key) {
            Some(styles) => { reduce_styles_into_style(styles, style); },
            None => {}
        }
    }
}

/// This takes a list of styles, and a mutable style object, and attempts to configure the
/// style object in a way that makes sense given n styles.
fn reduce_styles_into_style(styles: &Vec<Styles>, layout: &mut Style) {
    for style in styles { match style {
        Styles::AlignContent(val) => { layout.align_content = *val; },
        Styles::AlignItems(val) => { layout.align_items = *val; },
        Styles::AlignSelf(val) => { layout.align_self = *val; },
        Styles::AspectRatio(val) => { layout.aspect_ratio = *val; },
        Styles::BackfaceVisibility(_val) => { },
        Styles::BackgroundColor(val) => { layout.background_color = *val; },

        Styles::BorderColor(_val) => { },
        Styles::BorderEndColor(_val) => { },
        Styles::BorderBottomColor(_val) => { },
        Styles::BorderLeftColor(_val) => { },
        Styles::BorderRightColor(_val) => { },
        Styles::BorderTopColor(_val) => { },
        Styles::BorderStartColor(_val) => { },
    
        Styles::BorderStyle(_val) => { },
        Styles::BorderEndStyle(_val) => { },
        Styles::BorderBottomStyle(_val) => { },
        Styles::BorderLeftStyle(_val) => { },
        Styles::BorderRightStyle(_val) => { },
        Styles::BorderTopStyle(_val) => { },
        Styles::BorderStartStyle(_val) => { },
    
        Styles::BorderWidth(_val) => { },
        Styles::BorderEndWidth(_val) => { },
        Styles::BorderBottomWidth(_val) => { },
        Styles::BorderLeftWidth(_val) => { },
        Styles::BorderRightWidth(_val) => { },
        Styles::BorderTopWidth(_val) => { },
        Styles::BorderStartWidth(_val) => { },

        Styles::BorderRadius(_val) => { },
        Styles::BorderBottomEndRadius(_val) => { },
        Styles::BorderBottomLeftRadius(_val) => { },
        Styles::BorderBottomRightRadius(_val) => { },
        Styles::BorderBottomStartRadius(_val) => { },
        Styles::BorderTopLeftRadius(_val) => { },
        Styles::BorderTopRightRadius(_val) => { },
        Styles::BorderTopEndRadius(_val) => { },
        Styles::BorderTopStartRadius(_val) => { },
    
        Styles::Bottom(val) => {
            layout.position = Rect {
                start: layout.position.start,
                end: layout.position.end,
                top: layout.position.top,
                bottom: Dimension::Points(*val)
            };
        },

        Styles::Direction(val) => { layout.direction = *val; },
        Styles::Display(val) => { layout.display = *val; },
        
        Styles::End(val) => {
            layout.position = Rect {
                start: layout.position.start,
                end: Dimension::Points(*val),
                top: layout.position.top,
                bottom: layout.position.bottom
            };
        },
        
        Styles::FlexBasis(val) => { layout.flex_basis = Dimension::Points(*val); },
        Styles::FlexDirection(val) => { layout.flex_direction = *val; },
        Styles::FlexGrow(val) => { layout.flex_grow = *val; },
        Styles::FlexShrink(val) => { layout.flex_shrink = *val; },
        Styles::FlexWrap(val) => { layout.flex_wrap = *val; },
        
        Styles::FontFamily(_val) => { },
        Styles::FontLineHeight(_val) => { },
        Styles::FontSize(_val) => { },
        Styles::FontStyle(_val) => { },
        Styles::FontWeight(_val) => { },
        
        Styles::Height(val) => {
            layout.size = Size {
                width: layout.size.width,
                height: Dimension::Points(*val)
            };
        },

        Styles::JustifyContent(val) => { layout.justify_content = *val; },

        Styles::Left(val) => {
            layout.position = Rect {
                start: Dimension::Points(*val),
                end: layout.position.end,
                top: layout.position.top,
                bottom: layout.position.bottom
            };
        },

        Styles::MarginBottom(val) => {
            layout.margin = Rect {
                start: layout.margin.start,
                end: layout.margin.end,
                top: layout.margin.top,
                bottom: Dimension::Points(*val)
            };
        },

        Styles::MarginEnd(val) => {
            layout.margin = Rect {
                start: layout.margin.start,
                end: Dimension::Points(*val),
                top: layout.margin.top,
                bottom: layout.margin.bottom
            };
        },

        Styles::MarginLeft(val) => {
            layout.margin = Rect {
                start: Dimension::Points(*val),
                end: layout.margin.end,
                top: layout.margin.top,
                bottom: layout.margin.bottom
            };
        },

        Styles::MarginRight(val) => {
            layout.margin = Rect {
                start: layout.margin.start,
                end: Dimension::Points(*val),
                top: layout.margin.top,
                bottom: layout.margin.bottom
            };
        },
        
        Styles::MarginStart(val) => {
            layout.margin = Rect {
                start: Dimension::Points(*val),
                end: layout.margin.end,
                top: layout.margin.top,
                bottom: layout.margin.bottom
            };
        },

        Styles::MarginTop(val) => {
            layout.margin = Rect {
                start: layout.margin.start,
                end: layout.margin.end,
                top: Dimension::Points(*val),
                bottom: layout.margin.bottom
            };
        },

        Styles::MaxHeight(val) => {
            layout.max_size = Size {
                width: layout.max_size.width,
                height: Dimension::Points(*val)
            };
        },

        Styles::MaxWidth(val) => {
            layout.max_size = Size {
                width: Dimension::Points(*val),
                height: layout.max_size.height
            };
        },

        Styles::MinHeight(val) => {
            layout.min_size = Size {
                width: layout.min_size.width,
                height: Dimension::Points(*val)
            };
        },

        Styles::MinWidth(val) => {
            layout.min_size = Size {
                width: Dimension::Points(*val),
                height: layout.min_size.height
            };
        },

        Styles::Opacity(val) => { },
        Styles::Overflow(val) => { },

        Styles::PaddingBottom(val) => {
            layout.padding = Rect {
                start: layout.padding.start,
                end: layout.padding.end,
                top: layout.padding.top,
                bottom: Dimension::Points(*val)
            };
        },

        Styles::PaddingEnd(val) => {
            layout.padding = Rect {
                start: layout.padding.start,
                end: Dimension::Points(*val),
                top: layout.padding.top,
                bottom: layout.padding.bottom
            };
        },

        Styles::PaddingLeft(val) => {
            layout.padding = Rect {
                start: Dimension::Points(*val),
                end: layout.padding.end,
                top: layout.padding.top,
                bottom: layout.padding.bottom
            };
        },
        
        Styles::PaddingRight(val) => {
            layout.padding = Rect {
                start: layout.padding.start,
                end: Dimension::Points(*val),
                top: layout.padding.top,
                bottom: layout.padding.bottom
            };
        },

        Styles::PaddingStart(val) => {
            layout.padding = Rect {
                start: Dimension::Points(*val),
                end: layout.padding.end,
                top: layout.padding.top,
                bottom: layout.padding.bottom
            };
        },

        Styles::PaddingTop(val) => {
            layout.padding = Rect {
                start: layout.padding.start,
                end: layout.padding.end,
                top: Dimension::Points(*val),
                bottom: layout.padding.bottom
            };
        },

        Styles::PositionType(val) => { layout.position_type = *val; },

        Styles::Right(val) => {
            layout.position = Rect {
                start: layout.position.start,
                end: Dimension::Points(*val),
                top: layout.position.top,
                bottom: layout.position.bottom
            };
        },
        
        Styles::Start(val) => {
            layout.position = Rect {
                start: Dimension::Points(*val),
                end: layout.position.end,
                top: layout.position.top,
                bottom: layout.position.bottom
            };
        },
        
        Styles::TextAlignment(val) => { },
        Styles::TextColor(val) => { layout.text_color = *val; },
        Styles::TextDecorationColor(val) => { },
        Styles::TextShadowColor(val) => { },
        Styles::TintColor(val) => { },
        
        Styles::Top(val) => {
            layout.position = Rect {
                start: layout.position.start,
                end: layout.position.end,
                top: Dimension::Points(*val),
                bottom: layout.position.bottom
            };
        },
        
        Styles::Width(val) => {
            layout.size = Size {
                width: Dimension::Points(*val),
                height: layout.size.height
            };
        }
    }}
}
