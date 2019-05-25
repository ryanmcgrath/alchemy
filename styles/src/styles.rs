/// Implements the various `Style` types used for computing Flexbox layouts,
/// along with appearance-based styles (`Color`s, etc).

#[cfg(feature="tokenize")]
use proc_macro2::{TokenStream, Ident, Span};

#[cfg(feature="tokenize")]
use quote::{quote, ToTokens};

pub use crate::geometry::{Rect, Size};
pub use crate::number::Number;
pub use crate::color::Color;
pub use crate::stretch::result::Layout;

/// Describes how items should be aligned.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Default for AlignItems {
    fn default() -> AlignItems {
        AlignItems::Stretch
    }
}

/// Describes how this item should be aligned.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Default for AlignSelf {
    fn default() -> AlignSelf {
        AlignSelf::Auto
    }
}

/// Describes how content should be aligned.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

impl Default for AlignContent {
    fn default() -> AlignContent {
        AlignContent::Stretch
    }
}

/// Describes how things should flow - particularly important for start/end positions.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    Inherit,
    LTR,
    RTL,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Inherit
    }
}

/// Describes whether an item is visible or not.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Display {
    Flex,
    None,
}

impl Default for Display {
    fn default() -> Display {
        Display::Flex
    }
}

/// Describes how items should be aligned.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> FlexDirection {
        FlexDirection::Row
    }
}

impl FlexDirection {
    /// Checks if this is a row.
    pub(crate) fn is_row(self) -> bool {
        self == FlexDirection::Row || self == FlexDirection::RowReverse
    }

    /// Checks if this is a column.
    pub(crate) fn is_column(self) -> bool {
        self == FlexDirection::Column || self == FlexDirection::ColumnReverse
    }

    /// Checks if this is a reversed direction.
    pub(crate) fn is_reverse(self) -> bool {
        self == FlexDirection::RowReverse || self == FlexDirection::ColumnReverse
    }
}

/// Describes how content should be justified.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> JustifyContent {
        JustifyContent::FlexStart
    }
}

/// Describes how content should overflow.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

impl Default for Overflow {
    fn default() -> Overflow {
        Overflow::Visible
    }
}

/// Describes how content should be positioned.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PositionType {
    Relative,
    Absolute,
}

impl Default for PositionType {
    fn default() -> PositionType {
        PositionType::Relative
    }
}

/// Describes how content should wrap.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> FlexWrap {
        FlexWrap::NoWrap
    }
}

/// Describes a Dimension; automatic, undefined, or a value.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dimension {
    Undefined,
    Auto,
    Points(f32),
    Percent(f32),
}

impl Default for Dimension {
    fn default() -> Dimension {
        Dimension::Undefined
    }
}

impl Dimension {
    /// Internal method for Stretch. 
    pub(crate) fn resolve(self, parent_width: Number) -> Number {
        match self {
            Dimension::Points(points) => Number::Defined(points),
            Dimension::Percent(percent) => parent_width * percent,
            _ => Number::Undefined,
        }
    }

    /// Whether this Dimension is defined by a value or not.
    pub(crate) fn is_defined(self) -> bool {
        match self {
            Dimension::Points(_) => true,
            Dimension::Percent(_) => true,
            _ => false,
        }
    }
}

impl Default for Rect<Dimension> {
    fn default() -> Rect<Dimension> {
        Rect { start: Default::default(), end: Default::default(), top: Default::default(), bottom: Default::default() }
    }
}

impl Default for Size<Dimension> {
    fn default() -> Size<Dimension> {
        Size { width: Dimension::Auto, height: Dimension::Auto }
    }
}

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

/// `Style` is passed into the Stretch Flexbox rendering system to produce a computed
/// `Layout`. This is also passed to native nodes, to transform into per-platform style
/// commands.
#[derive(Copy, Clone, Debug)]
pub struct Style {
    pub display: Display,
    pub position_type: PositionType,
    pub direction: Direction,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub overflow: Overflow,
    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub align_content: AlignContent,
    pub justify_content: JustifyContent,
    pub position: Rect<Dimension>,
    pub margin: Rect<Dimension>,
    pub padding: Rect<Dimension>,
    pub border: Rect<Dimension>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,
    pub size: Size<Dimension>,
    pub min_size: Size<Dimension>,
    pub max_size: Size<Dimension>,
    pub aspect_ratio: Number,

    // Appearance-based styles
    pub background_color: Color,
    pub text_color: Color
}

impl Default for Style {
    fn default() -> Style {
        Style {
            display: Default::default(),
            position_type: Default::default(),
            direction: Default::default(),
            flex_direction: Default::default(),
            flex_wrap: Default::default(),
            overflow: Default::default(),
            align_items: Default::default(),
            align_self: Default::default(),
            align_content: Default::default(),
            justify_content: Default::default(),
            position: Default::default(),
            margin: Default::default(),
            padding: Default::default(),
            border: Default::default(),
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Dimension::Auto,
            size: Default::default(),
            min_size: Default::default(),
            max_size: Default::default(),
            aspect_ratio: Default::default(),
            background_color: Color::transparent(),
            text_color: Color::transparent()
        }
    }
}

impl Style {
    /// Determines the minimum main size, given flex direction.
    pub(crate) fn min_main_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.min_size.width,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.min_size.height,
        }
    }

    /// Determines the maximum main size, given flex direction.
    pub(crate) fn max_main_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.max_size.width,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.max_size.height,
        }
    }

    /// Determines the main margin start, given flex direction.
    pub(crate) fn main_margin_start(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.start,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.top,
        }
    }

    /// Determines the main margin end, given flex direction.
    pub(crate) fn main_margin_end(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.end,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.bottom,
        }
    }

    /// Determines the cross size, given flex direction.
    pub(crate) fn cross_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.size.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.size.width,
        }
    }

    /// Determines the minimum cross size, given flex direction.
    pub(crate) fn min_cross_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.min_size.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.min_size.width,
        }
    }

    /// Determines the maximum cross size, given flex direction.
    pub(crate) fn max_cross_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.max_size.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.max_size.width,
        }
    }

    /// Determines the cross margin start, given flex direction.
    pub(crate) fn cross_margin_start(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.top,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.start,
        }
    }

    /// Determines the cross margin end, given flex direction.
    pub(crate) fn cross_margin_end(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.bottom,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.end,
        }
    }

    /// Determines the inherited align_self style, given a parent `&Style`.
    pub(crate) fn align_self(&self, parent: &Style) -> AlignSelf {
        if self.align_self == AlignSelf::Auto {
            match parent.align_items {
                AlignItems::FlexStart => AlignSelf::FlexStart,
                AlignItems::FlexEnd => AlignSelf::FlexEnd,
                AlignItems::Center => AlignSelf::Center,
                AlignItems::Baseline => AlignSelf::Baseline,
                AlignItems::Stretch => AlignSelf::Stretch,
            }
        } else {
            self.align_self
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
