//! CSS parsing logic. Mostly relies from the rust-cssparser crate,
//! slightly modified to fit the `Styles` structure we want internally.

use cssparser::{
    AtRuleParser, BasicParseError, CowRcStr,
    DeclarationListParser, DeclarationParser,
    Parser, ParseError, QualifiedRuleParser,
    SourceLocation, Token
};

use crate::styles::*;

/// Represents a style rule, a `key: [values...];` pair.
#[derive(Debug)]
pub struct Rule {
    pub key: String,
    pub styles: Vec<Styles>
}

/// The parser itself.
#[derive(Debug)]
pub struct RuleParser;

/// Some type information for our parser.
impl<'i> AtRuleParser<'i> for RuleParser {
    type PreludeBlock = ();
    type PreludeNoBlock = ();
    type AtRule = Rule;
    type Error = BasicParseError<'i>;
}

/// The actual work our parser does. Walks style rules and attempts to
/// extract the key/value pairings from a given stylesheet string.
impl<'i> QualifiedRuleParser<'i> for RuleParser {
    type Prelude = String;
    type QualifiedRule = Rule;
    type Error = BasicParseError<'i>;

    /// Parses out the selector.
    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let location = input.current_source_location();

        let selector = match input.next()? {
            Token::Ident(ref element_name) => element_name.to_string(),
            t => { return Err(location.new_unexpected_token_error(t.clone())); }
        };

        // If there's a next, someone is writing their code assuming cascading. Let's... warn them.
        /*match input.next()? {
            Ok(_) => {},
            Err(e) => {}
        };*/

        Ok(selector)
    }

    /// Parses the block (`{...}`) into a Rule struct.
    fn parse_block<'t>(
        &mut self,
        key: Self::Prelude,
        _location: SourceLocation,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let styles = DeclarationListParser::new(input, StyleParser {}).collect::<Vec<_>>();

        Ok(Rule {
            key: key,
            styles: styles.into_iter().filter_map(|decl| {
                if !decl.is_ok() {
                    eprintln!("{:?}", decl);
                }

                decl.ok()
            }).collect()
        })
    }
}

/// Contains logic for matching CSS attributes to their `Styles` counterpart.
#[derive(Debug)]
pub struct StyleParser;

/// Types, etc.
impl<'i> AtRuleParser<'i> for StyleParser {
    type PreludeBlock = ();
    type PreludeNoBlock = ();
    type AtRule = Styles;
    type Error = BasicParseError<'i>;
}

/// A utility method for dereferencing a value, to make some code later on a bit more clean.
fn ident<'a>(token: &'a Token) -> &'a str {
    match token {
        Token::Ident(ref value) => &*value,
        _ => ""
    }
}

impl<'i> DeclarationParser<'i> for StyleParser {
    type Declaration = Styles;
    type Error = BasicParseError<'i>;

    /// Parses a value (e.g, `background-color: #307ace;`) into a `Styles` value.
    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let style = match &*name {
            "align-content" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "flex-start" => Styles::AlignContent(AlignContent::FlexStart),
                "flex-end" => Styles::AlignContent(AlignContent::FlexEnd),
                "center" => Styles::AlignContent(AlignContent::Center),
                "stretch" => Styles::AlignContent(AlignContent::Stretch),
                "space-between" => Styles::AlignContent(AlignContent::SpaceBetween),
                "space-around" => Styles::AlignContent(AlignContent::SpaceAround),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},
            
            "align-items" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "flex-start" => Styles::AlignItems(AlignItems::FlexStart),
                "flex-end" => Styles::AlignItems(AlignItems::FlexEnd),
                "center" => Styles::AlignItems(AlignItems::Center),
                "baseline" => Styles::AlignItems(AlignItems::Baseline),
                "stretch" => Styles::AlignItems(AlignItems::Stretch),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},
            
            "align_self" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "auto" => Styles::AlignSelf(AlignSelf::Auto),
                "flex-start" => Styles::AlignSelf(AlignSelf::FlexStart),
                "flex-end" => Styles::AlignSelf(AlignSelf::FlexEnd),
                "center" => Styles::AlignSelf(AlignSelf::Center),
                "baseline" => Styles::AlignSelf(AlignSelf::Baseline),
                "stretch" => Styles::AlignSelf(AlignSelf::Stretch),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},

            // @TODO: Aspect Ratio... could be string, no? Should this be handled better?
            "aspect-ratio" => Styles::AspectRatio(Number::Defined(parse_floaty_mcfloatface_value(input)?)),

            "backface-visibility" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "visible" => Styles::BackfaceVisibility(BackfaceVisibility::Visible),
                "hidden" => Styles::BackfaceVisibility(BackfaceVisibility::Hidden),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},

            "background-color" => Styles::BackgroundColor(Color::parse(input)?),
            
            // Border values~
            "border-color" => Styles::BorderColor(Color::parse(input)?),
            "border-top-color" => Styles::BorderTopColor(Color::parse(input)?),
            "border-bottom-color" => Styles::BorderBottomColor(Color::parse(input)?),
            "border-left-color" => Styles::BorderLeftColor(Color::parse(input)?),
            "border-right-color" => Styles::BorderRightColor(Color::parse(input)?),
            
            "bottom" => Styles::Bottom(parse_floaty_mcfloatface_value(input)?),

            "color" => Styles::TextColor(Color::parse(input)?),

            "direction" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "inherit" => Styles::Direction(Direction::Inherit),
                "ltr" => Styles::Direction(Direction::LTR),
                "rtl" => Styles::Direction(Direction::RTL),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},

            "display" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "flex" => Styles::Display(Display::Flex),
                "none" => Styles::Display(Display::None),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},
            
            "end" => Styles::End(parse_floaty_mcfloatface_value(input)?),

            "flex-basis" => Styles::FlexBasis(parse_floaty_mcfloatface_value(input)?),
            
            "flex-direction" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "row" => Styles::FlexDirection(FlexDirection::Row),
                "row-reverse" => Styles::FlexDirection(FlexDirection::RowReverse),
                "column" => Styles::FlexDirection(FlexDirection::Column),
                "column-reverse" => Styles::FlexDirection(FlexDirection::ColumnReverse),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},

            "flex-grow" => Styles::FlexGrow(parse_floaty_mcfloatface_value(input)?),
            "flex-shrink" => Styles::FlexShrink(parse_floaty_mcfloatface_value(input)?),
            
            "flex-wrap" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "no-wrap" => Styles::FlexWrap(FlexWrap::NoWrap),
                "wrap" => Styles::FlexWrap(FlexWrap::Wrap),
                "wrap-reverse" => Styles::FlexWrap(FlexWrap::WrapReverse),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},
            
            //FontFamily(FontFamily),
            "font-size" => Styles::FontSize(parse_floaty_mcfloatface_value(input)?),
            
            "font-style" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "normal" => Styles::FontStyle(FontStyle::Normal),
                "italic" => Styles::FontStyle(FontStyle::Italic),
                "oblique" => Styles::FontStyle(FontStyle::Oblique),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},

            "font-weight" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "normal" => Styles::FontWeight(FontWeight::Normal),
                "bold" => Styles::FontWeight(FontWeight::Bold),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},
            
            "height" => Styles::Height(parse_floaty_mcfloatface_value(input)?),

            "justify-content" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "flex-start" => Styles::JustifyContent(JustifyContent::FlexStart),
                "flex-end" => Styles::JustifyContent(JustifyContent::FlexEnd),
                "center" => Styles::JustifyContent(JustifyContent::Center),
                "space-between" => Styles::JustifyContent(JustifyContent::SpaceBetween),
                "space-around" => Styles::JustifyContent(JustifyContent::SpaceAround),
                "space-evenly" => Styles::JustifyContent(JustifyContent::SpaceEvenly),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},
            
            "left" => Styles::Left(parse_floaty_mcfloatface_value(input)?),
            "line-height" => Styles::FontLineHeight(parse_floaty_mcfloatface_value(input)?),

            "margin-bottom" => Styles::MarginBottom(parse_floaty_mcfloatface_value(input)?),
            "margin-end" => Styles::MarginEnd(parse_floaty_mcfloatface_value(input)?),
            "margin-left" => Styles::MarginLeft(parse_floaty_mcfloatface_value(input)?),
            "margin-right" => Styles::MarginRight(parse_floaty_mcfloatface_value(input)?),
            "margin-start" => Styles::MarginStart(parse_floaty_mcfloatface_value(input)?),
            "margin-top" => Styles::MarginTop(parse_floaty_mcfloatface_value(input)?),

            "max-height" => Styles::MaxHeight(parse_floaty_mcfloatface_value(input)?),
            "max-width" => Styles::MaxWidth(parse_floaty_mcfloatface_value(input)?),
            
            "min-height" => Styles::MinHeight(parse_floaty_mcfloatface_value(input)?),
            "min-width" => Styles::MinWidth(parse_floaty_mcfloatface_value(input)?),

            "opacity" => Styles::Opacity(parse_floaty_mcfloatface_value(input)?),
            
            "overflow" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "visible" => Styles::Overflow(Overflow::Visible),
                "hidden" => Styles::Overflow(Overflow::Hidden),
                "scroll" => Styles::Overflow(Overflow::Scroll),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},
            
            "padding-bottom" => Styles::PaddingBottom(parse_floaty_mcfloatface_value(input)?),
            "padding-end" => Styles::PaddingEnd(parse_floaty_mcfloatface_value(input)?),
            "padding-left" => Styles::PaddingLeft(parse_floaty_mcfloatface_value(input)?),
            "padding-right" => Styles::PaddingRight(parse_floaty_mcfloatface_value(input)?),
            "padding-start" => Styles::PaddingStart(parse_floaty_mcfloatface_value(input)?),
            "padding-top" => Styles::PaddingTop(parse_floaty_mcfloatface_value(input)?),
            
            "position" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "absolute" => Styles::PositionType(PositionType::Absolute),
                "relative" => Styles::PositionType(PositionType::Relative),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},
            
            "right" => Styles::Right(parse_floaty_mcfloatface_value(input)?),
            "start" => Styles::Start(parse_floaty_mcfloatface_value(input)?),
            
            "text-align" => { let s = input.current_source_location(); let t = input.next()?; match ident(&t) {
                "auto" => Styles::TextAlignment(TextAlignment::Auto),
                "left" => Styles::TextAlignment(TextAlignment::Left),
                "right" => Styles::TextAlignment(TextAlignment::Right),
                "center" => Styles::TextAlignment(TextAlignment::Center),
                "justify" => Styles::TextAlignment(TextAlignment::Justify),
                _ => { return Err(s.new_unexpected_token_error(t.clone())); }
            }},
            
            "text-decoration-color" => Styles::TextDecorationColor(Color::parse(input)?),
            "text-shadow-color" => Styles::TextShadowColor(Color::parse(input)?),
            "tint-color" => Styles::TintColor(Color::parse(input)?),
            
            "top" => Styles::Top(parse_floaty_mcfloatface_value(input)?),
            "width" => Styles::Width(parse_floaty_mcfloatface_value(input)?),
            
            t => {
                let location = input.current_source_location();
                return Err(location.new_unexpected_token_error(Token::Ident(t.to_string().into())));
            }
        };

        Ok(style)
    }
}

/// A utility method for handling some float values.
/// Mostly used to reduce code verbosity in the massive switch table for `Styles` parsing.
fn parse_floaty_mcfloatface_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<f32, BasicParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;

    match token {
        Token::Number { value, .. } => Ok(*value),    
        _ => Err(location.new_basic_unexpected_token_error(token.clone()))
    }
}
