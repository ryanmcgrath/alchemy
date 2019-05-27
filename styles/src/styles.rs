/// Implements the various `Style` types used for computing Flexbox layouts,
/// along with appearance-based styles (`Color`s, etc).

#[cfg(feature="tokenize")]
use proc_macro2::{TokenStream, Ident, Span};

#[cfg(feature="tokenize")]
use quote::{quote, ToTokens};

pub use crate::color::Color;

pub use crate::stretch::geometry::{Point, Rect, Size};
pub use crate::stretch::number::Number;
pub use crate::stretch::result::Layout;

pub use crate::stretch::style::{
    Style,
    AlignContent, AlignItems, AlignSelf, Dimension, Direction, Display,
    FlexDirection, JustifyContent, Overflow, PositionType, FlexWrap
};

/// Describes the backface-visibility for a view. This may be removed in a later release.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BackfaceVisibility {
    Visible,
    Hidden
}

impl Default for BackfaceVisibility {
    fn default() -> BackfaceVisibility {
        BackfaceVisibility::Visible
    }
}

/// Describes a font style.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique
}

impl Default for FontStyle {
    fn default() -> FontStyle {
        FontStyle::Normal
    }
}

/// Describes a font weight.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FontWeight {
    Normal,
    Bold
}

impl Default for FontWeight {
    fn default() -> FontWeight {
        FontWeight::Normal
    }
}

/// Describes how text should be aligned.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TextAlignment {
    Auto,
    Left,
    Right,
    Center,
    Justify
}

impl Default for TextAlignment {
    fn default() -> TextAlignment {
        TextAlignment::Auto
    }
}

/// Describes a border style.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BorderStyle {
    None, // The CSS value is None, but it's a reserved term in Rust ;P
    Hidden,
    Solid
}

impl Default for BorderStyle {
    fn default() -> BorderStyle {
        BorderStyle::None
    }
}

/// Describes how a Font Family
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FontFamily {
    SansSerif // @TODO This is tricky because of &str/String/Copy. Revisit later.
}

impl Default for FontFamily {
    fn default() -> Self {
        FontFamily::SansSerif
    }
}

/// When applying layout to a backing view, you'll get two calls - one with a `Layout`, 
/// which contains the computed frame, and one with an `Appearance`, which contains things 
/// like colors, fonts, and so on.
pub struct Appearance {
    pub background_color: Color,
    pub font_size: f32,
    pub font_style: FontStyle,
    pub font_weight: FontWeight,
    pub opacity: f32,
    pub text_alignment: TextAlignment,
    pub text_color: Color,
    pub text_decoration_color: Color,
    pub text_shadow_color: Color,
    pub tint_color: Color
}

impl Default for Appearance {
    fn default() -> Appearance {
        Appearance {
            background_color: Color::transparent(),
            // @TODO: We can definitely judge a default value better here. 
            font_size: 14.,
            font_style: FontStyle::default(),
            font_weight: FontWeight::default(),
            opacity: 1.,
            text_alignment: TextAlignment::default(),
            text_color: Color::transparent(),
            text_decoration_color: Color::transparent(),
            text_shadow_color: Color::transparent(),
            tint_color: Color::transparent()
        }
    }
}

/// These exist purely for use in the parser code.
///
/// A `Style` is what's used for a node; `Styles` are what's parsed and stored.
/// At render-time, the rendering engine takes n styles and reduces them down into 1 `Style`
/// that's applied to the node in question.
#[derive(Debug)]
pub enum Styles {
    AlignContent(AlignContent),
    AlignItems(AlignItems),
    AlignSelf(AlignSelf),
    AspectRatio(Number),
    BackfaceVisibility(BackfaceVisibility),
    BackgroundColor(Color),

    BorderColor(Color),
    BorderEndColor(Color),
    BorderBottomColor(Color),
    BorderLeftColor(Color),
    BorderRightColor(Color),
    BorderTopColor(Color),
    BorderStartColor(Color),
    
    BorderStyle(BorderStyle),
    BorderEndStyle(BorderStyle),
    BorderBottomStyle(BorderStyle),
    BorderLeftStyle(BorderStyle),
    BorderRightStyle(BorderStyle),
    BorderTopStyle(BorderStyle),
    BorderStartStyle(BorderStyle),
    
    BorderWidth(f32),
    BorderEndWidth(f32),
    BorderBottomWidth(f32),
    BorderLeftWidth(f32),
    BorderRightWidth(f32),
    BorderTopWidth(f32),
    BorderStartWidth(f32),

    BorderRadius(f32),
    BorderBottomEndRadius(f32),
    BorderBottomLeftRadius(f32),
    BorderBottomRightRadius(f32),
    BorderBottomStartRadius(f32),
    BorderTopLeftRadius(f32),
    BorderTopRightRadius(f32),
    BorderTopEndRadius(f32),
    BorderTopStartRadius(f32),
    
    Bottom(f32),
    Direction(Direction),
    Display(Display),
    End(f32),
    FlexBasis(f32),
    FlexDirection(FlexDirection),
    FlexGrow(f32),
    FlexShrink(f32),
    FlexWrap(FlexWrap),
    FontFamily(FontFamily),
    FontLineHeight(f32),
    FontSize(f32),
    FontStyle(FontStyle),
    FontWeight(FontWeight),
    Height(f32),
    JustifyContent(JustifyContent),
    Left(f32),
    MarginBottom(f32),
    MarginEnd(f32),
    MarginLeft(f32),
    MarginRight(f32),
    MarginStart(f32),
    MarginTop(f32),
    MaxHeight(f32),
    MaxWidth(f32),
    MinHeight(f32),
    MinWidth(f32),
    Opacity(f32),
    Overflow(Overflow),
    PaddingBottom(f32),
    PaddingEnd(f32),
    PaddingLeft(f32),
    PaddingRight(f32),
    PaddingStart(f32),
    PaddingTop(f32),
    PositionType(PositionType),
    Right(f32),
    Start(f32),
    TextAlignment(TextAlignment),
    TextColor(Color),
    TextDecorationColor(Color),
    TextShadowColor(Color),
    TintColor(Color),
    Top(f32),
    Width(f32)
}

/// A method for tokenizing a `Color` for a given attribute (e.g, `BackgroundColor`).
#[cfg(feature="tokenize")]
fn color_tokens(tokens: &mut TokenStream, color: &Color, style: &str) {
    let red = color.red;
    let green = color.green;
    let blue = color.blue;
    let alpha = color.alpha;
    let s = Ident::new(style, Span::call_site());

    tokens.extend(quote!(Styles::#s(Color {
        red: #red,
        green: #green,
        blue: #blue,
        alpha: #alpha
    })));
}

/// Converts `Styles` into tokenized `Styles` representations, for use in the `styles! {}` macro.
#[cfg(feature="tokenize")]
impl ToTokens for Styles {
    fn to_tokens(&self, tokens: &mut TokenStream) { match self {
        Styles::AlignContent(align_content) => { match align_content {
            AlignContent::FlexStart => tokens.extend(quote!(Styles::AlignContent(AlignContent::FlexStart))),
            AlignContent::FlexEnd => tokens.extend(quote!(Styles::AlignContent(AlignContent::FlexEnd))),
            AlignContent::Center => tokens.extend(quote!(Styles::AlignContent(AlignContent::Center))),
            AlignContent::Stretch => tokens.extend(quote!(Styles::AlignContent(AlignContent::Stretch))),
            AlignContent::SpaceAround => tokens.extend(quote!(Styles::AlignContent(AlignContent::SpaceAround))),
            AlignContent::SpaceBetween => tokens.extend(quote!(Styles::AlignContent(AlignContent::SpaceBetween)))
        }},

        Styles::AlignItems(align_items) => { match align_items {
            AlignItems::FlexStart => tokens.extend(quote!(Styles::AlignItems(AlignItems::FlexStart))),
            AlignItems::FlexEnd => tokens.extend(quote!(Styles::AlignItems(AlignItems::FlexEnd))),
            AlignItems::Center => tokens.extend(quote!(Styles::AlignItems(AlignItems::Center))),
            AlignItems::Baseline => tokens.extend(quote!(Styles::AlignItems(AlignItems::Baseline))),
            AlignItems::Stretch => tokens.extend(quote!(Styles::AlignItems(AlignItems::Stretch)))
        }},

        Styles::AlignSelf(align_self) => { match align_self {
            AlignSelf::Auto => tokens.extend(quote!(Styles::AlignSelf(AlignSelf::Auto))),
            AlignSelf::FlexStart => tokens.extend(quote!(Styles::AlignSelf(AlignSelf::FlexStart))),
            AlignSelf::FlexEnd => tokens.extend(quote!(Styles::AlignSelf(AlignSelf::FlexEnd))),
            AlignSelf::Center => tokens.extend(quote!(Styles::AlignSelf(AlignSelf::Center))),
            AlignSelf::Baseline => tokens.extend(quote!(Styles::AlignSelf(AlignSelf::Baseline))),
            AlignSelf::Stretch => tokens.extend(quote!(Styles::AlignSelf(AlignSelf::Stretch)))
        }},

        Styles::AspectRatio(_) => {},
        
        Styles::BackfaceVisibility(visibility) => { match visibility {
            BackfaceVisibility::Visible => tokens.extend(quote!(Styles::BackfaceVisibility(BackfaceVisibility::Visible))),
            BackfaceVisibility::Hidden => tokens.extend(quote!(Styles::BackfaceVisibility(BackfaceVisibility::Hidden)))
        }},
        
        Styles::BackgroundColor(color) => color_tokens(tokens, color, "BackgroundColor"),
        Styles::BorderColor(color) => color_tokens(tokens, color, "BorderColor"),
        Styles::BorderEndColor(color) => color_tokens(tokens, color, "BorderEndColor"),
        Styles::BorderBottomColor(color) => color_tokens(tokens, color, "BorderBottomColor"),
        Styles::BorderLeftColor(color) => color_tokens(tokens, color, "BorderLeftColor"),
        Styles::BorderRightColor(color) => color_tokens(tokens, color, "BorderRightColor"),
        Styles::BorderTopColor(color) => color_tokens(tokens, color, "BorderTopColor"),
        Styles::BorderStartColor(color) => color_tokens(tokens, color, "BorderStartColor"),
        Styles::BorderStyle(_) => {},
        Styles::BorderEndStyle(_) => {},
        Styles::BorderBottomStyle(_) => {},
        Styles::BorderLeftStyle(_) => {},
        Styles::BorderRightStyle(_) => {},
        Styles::BorderTopStyle(_) => {},
        Styles::BorderStartStyle(_) => {},
        Styles::BorderWidth(border_width) => tokens.extend(quote!(Styles::BorderWidth(#border_width))),
        Styles::BorderEndWidth(border_end_width) => tokens.extend(quote!(Styles::BorderEndWidth(#border_end_width))),
        Styles::BorderBottomWidth(border_bottom_width) => tokens.extend(quote!(Styles::BorderBottomWidth(#border_bottom_width))),
        Styles::BorderLeftWidth(border_left_width) => tokens.extend(quote!(Styles::BorderLeftWidth(#border_left_width))),
        Styles::BorderRightWidth(border_right_width) => tokens.extend(quote!(Styles::BorderRightWidth(#border_right_width))),
        Styles::BorderTopWidth(border_top_width) => tokens.extend(quote!(Styles::BorderTopWidth(#border_top_width))),
        Styles::BorderStartWidth(border_start_width) => tokens.extend(quote!(Styles::BorderStartWidth(#border_start_width))),
        Styles::BorderRadius(border_radius) => tokens.extend(quote!(Styles::BorderRadius(#border_radius))),
        Styles::BorderBottomEndRadius(border_bottom_end_radius) => tokens.extend(quote!(Styles::BorderBottomEndRadius(#border_bottom_end_radius))),
        Styles::BorderBottomLeftRadius(border_bottom_left_radius) => tokens.extend(quote!(Styles::BorderBottomLeftRadius(#border_bottom_left_radius))),
        Styles::BorderBottomRightRadius(border_bottom_right_radius) => tokens.extend(quote!(Styles::BorderBottomRightRadius(#border_bottom_right_radius))),
        Styles::BorderBottomStartRadius(border_bottom_start_radius) => tokens.extend(quote!(Styles::BorderBottomStartRadius(#border_bottom_start_radius))),
        Styles::BorderTopLeftRadius(border_top_left_radius) => tokens.extend(quote!(Styles::BorderTopLeftRadius(#border_top_left_radius))),
        Styles::BorderTopRightRadius(border_top_right_radius) => tokens.extend(quote!(Styles::BorderTopRightRadius(#border_top_right_radius))),
        Styles::BorderTopEndRadius(border_top_end_radius) => tokens.extend(quote!(Styles::BorderTopEndRadius(#border_top_end_radius))),
        Styles::BorderTopStartRadius(border_top_start_radius) => tokens.extend(quote!(Styles::BorderTopStartRadius(#border_top_start_radius))),
        Styles::Bottom(bottom) => tokens.extend(quote!(Styles::Bottom(#bottom))),
        
        Styles::Direction(direction) => { match direction {
            Direction::Inherit => tokens.extend(quote!(Styles::Direction(Direction::Inherit))),
            Direction::LTR => tokens.extend(quote!(Styles::Direction(Direction::LTR))),
            Direction::RTL => tokens.extend(quote!(Styles::Direction(Direction::RTL)))
        }},
        
        Styles::Display(display) => { match display {
            Display::Flex => tokens.extend(quote!(Styles::Display(Display::Flex))),
            Display::None => tokens.extend(quote!(Styles::Display(Display::None)))
        }},

        Styles::End(end) => tokens.extend(quote!(Styles::End(#end))),
        Styles::FlexBasis(flex_basis) => tokens.extend(quote!(Styles::FlexBasis(#flex_basis))),
        
        Styles::FlexDirection(direction) => { match direction {
            FlexDirection::Row => tokens.extend(quote!(Styles::FlexDirection(FlexDirection::Row))),
            FlexDirection::Column => tokens.extend(quote!(Styles::FlexDirection(FlexDirection::Column))),
            FlexDirection::RowReverse => tokens.extend(quote!(Styles::FlexDirection(FlexDirection::RowReverse))),
            FlexDirection::ColumnReverse => tokens.extend(quote!(Styles::FlexDirection(FlexDirection::ColumnReverse)))
        }},
        
        Styles::FlexGrow(flex_grow) => tokens.extend(quote!(Styles::FlexGrow(#flex_grow))),
        Styles::FlexShrink(flex_shrink) => tokens.extend(quote!(Styles::FlexShrink(#flex_shrink))),
        
        Styles::FlexWrap(wrap) => { match wrap {
            FlexWrap::NoWrap => tokens.extend(quote!(Styles::FlexWrap(FlexWrap::NoWrap))),
            FlexWrap::Wrap => tokens.extend(quote!(Styles::FlexWrap(FlexWrap::Wrap))),
            FlexWrap::WrapReverse => tokens.extend(quote!(Styles::FlexWrap(FlexWrap::WrapReverse)))
        }},
        
        Styles::FontFamily(_family) => {},
        Styles::FontLineHeight(line_height) => tokens.extend(quote!(Styles::LineHeight(#line_height))),
        Styles::FontSize(font_size) => tokens.extend(quote!(Styles::FontSize(#font_size))),
        Styles::FontStyle(_style) => {},
        Styles::FontWeight(_weight) => {},
        Styles::Height(height) => tokens.extend(quote!(Styles::Height(#height))),
        
        Styles::JustifyContent(justify) => { match justify {
            JustifyContent::FlexStart => tokens.extend(quote!(Styles::JustifyContent(JustifyContent::FlexStart))),
            JustifyContent::FlexEnd => tokens.extend(quote!(Styles::JustifyContent(JustifyContent::FlexEnd))),
            JustifyContent::Center => tokens.extend(quote!(Styles::JustifyContent(JustifyContent::Center))),
            JustifyContent::SpaceBetween => tokens.extend(quote!(Styles::JustifyContent(JustifyContent::SpaceBetween))),
            JustifyContent::SpaceAround => tokens.extend(quote!(Styles::JustifyContent(JustifyContent::SpaceAround))),
            JustifyContent::SpaceEvenly => tokens.extend(quote!(Styles::JustifyContent(JustifyContent::SpaceEvenly)))
        }},
        
        Styles::Left(left) => tokens.extend(quote!(Styles::Left(#left))),
        Styles::MarginBottom(margin_bottom) => tokens.extend(quote!(Styles::MarginBottom(#margin_bottom))),
        Styles::MarginEnd(margin_end) => tokens.extend(quote!(Styles::MarginEnd(#margin_end))),
        Styles::MarginLeft(margin_left) => tokens.extend(quote!(Styles::MarginLeft(#margin_left))),
        Styles::MarginRight(margin_right) => tokens.extend(quote!(Styles::MarginRight(#margin_right))),
        Styles::MarginStart(margin_start) => tokens.extend(quote!(Styles::MarginStart(#margin_start))),
        Styles::MarginTop(top) => tokens.extend(quote!(Styles::Top(#top))),
        Styles::MaxHeight(max_height) => tokens.extend(quote!(Styles::MaxHeight(#max_height))),
        Styles::MaxWidth(max_width) => tokens.extend(quote!(Styles::MaxWidth(#max_width))),
        Styles::MinHeight(min_height) => tokens.extend(quote!(Styles::MinHeight(#min_height))),
        Styles::MinWidth(min_width) => tokens.extend(quote!(Styles::MinWidth(#min_width))),
        Styles::Opacity(opacity) => tokens.extend(quote!(Styles::Opacity(#opacity))),
        
        Styles::Overflow(overflow) => { match overflow {
            Overflow::Visible => tokens.extend(quote!(Styles::Overflow(Overflow::Visible))),
            Overflow::Hidden => tokens.extend(quote!(Styles::Overflow(Overflow::Hidden))),
            Overflow::Scroll => tokens.extend(quote!(Styles::Overflow(Overflow::Scroll)))
        }},
        
        Styles::PaddingBottom(padding_bottom) => tokens.extend(quote!(Styles::PaddingBottom(#padding_bottom))),
        Styles::PaddingEnd(padding_end) => tokens.extend(quote!(Styles::PaddingEnd(#padding_end))),
        Styles::PaddingLeft(padding_left) => tokens.extend(quote!(Styles::PaddingLeft(#padding_left))),
        Styles::PaddingRight(padding_right) => tokens.extend(quote!(Styles::PaddingRight(#padding_right))),
        Styles::PaddingStart(padding_start) => tokens.extend(quote!(Styles::PaddingStart(#padding_start))),
        Styles::PaddingTop(padding_top) => tokens.extend(quote!(Styles::PaddingTop(#padding_top))),
        
        Styles::PositionType(position_type) => { match position_type {
            PositionType::Relative => tokens.extend(quote!(Styles::PositionType(PositionType::Relative))),
            PositionType::Absolute => tokens.extend(quote!(Styles::PositionType(PositionType::Absolute)))
        }},
        
        Styles::Right(right) => tokens.extend(quote!(Styles::Right(#right))),
        Styles::Start(start) => tokens.extend(quote!(Styles::Start(#start))),
        
        Styles::TextAlignment(alignment) => { match alignment {
            TextAlignment::Auto => tokens.extend(quote!(Styles::TextAlignment(TextAlignment::Auto))),
            TextAlignment::Left => tokens.extend(quote!(Styles::TextAlignment(TextAlignment::Left))),
            TextAlignment::Right => tokens.extend(quote!(Styles::TextAlignment(TextAlignment::Right))),
            TextAlignment::Center => tokens.extend(quote!(Styles::TextAlignment(TextAlignment::Center))),
            TextAlignment::Justify => tokens.extend(quote!(Styles::TextAlignment(TextAlignment::Justify)))
        }},

        Styles::TextColor(color) => color_tokens(tokens, color, "TextColor"),
        Styles::TextDecorationColor(color) => color_tokens(tokens, color, "TextDecorationColor"),
        Styles::TextShadowColor(color) => color_tokens(tokens, color, "TextShadowColor"),
        Styles::TintColor(color) => color_tokens(tokens, color, "TintColor"),
        Styles::Top(top) => tokens.extend(quote!(Styles::Top(#top))),
        Styles::Width(width) => tokens.extend(quote!(Styles::Width(#width)))
    }}
}
