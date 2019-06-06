#![recursion_limit = "128"]
#![cfg_attr(can_show_location_of_runtime_parse_error, feature(proc_macro_span))]

//! Implements macros used in Alchemy.
//!
//! - `rsx! {}`, which turns RSX tags into `RSX` node trees.
//! - `styles! {}`, which turns CSS stylesheet strings into `Vec<Styles>`.
//!
//! In general, you should prefer using these to constructing the above values manually.
//!
//! Much of the `rsx! {}` support is achieved by forking code riginally written by Bodil Stokke
//! over in [typed-html](https://github.com/bodil/typed-html).

extern crate proc_macro;

mod error;
mod rsx;
mod ident;
mod lexer;
mod map;
mod parser;
mod span;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2, Literal, Span};
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use alchemy_styles::cssparser::{Parser, ParserInput, RuleListParser};
use alchemy_styles::styles_parser::{Rule, RuleParser};

/// Implements the `rsx! {}` macro, which turns RSX tags into `RSX` node trees.
#[proc_macro_hack]
pub fn rsx(input: TokenStream) -> TokenStream {
    let stream = lexer::unroll_stream(input.into(), false);
    let result = rsx::expand_rsx(&stream);
    TokenStream::from(match result {
        Err(err) => error::parse_error(&stream, &err),
        Ok((node, ty)) => match node.into_token_stream(&ty) {
            Err(err) => err,
            Ok(success) => success,
        },
    })
}

/// Implements the `styles! {}` macro, which turns CSS stylesheet strings into `Vec<Styles>`.
#[proc_macro_hack]
pub fn styles(input: TokenStream) -> TokenStream {
    let s = input.to_string().replace(" ", "");
    let mut input = ParserInput::new(&s);
    let mut parser = Parser::new(&mut input);
        
    let parsed: Vec<Rule> = RuleListParser::new_for_stylesheet(&mut parser, RuleParser {})
        .collect::<Vec<_>>()
        .into_iter()
        .filter_map(|rule| {
            rule.ok()
        })
        .collect();

    let mut body = TokenStream2::new();
    for rule in parsed {
        let mut stream = TokenStream2::new();
        for style in rule.styles {
            stream.extend(quote!(#style,));
        }

        let key = Literal::string(&rule.key);
        body.extend(quote!(styles.insert(#key, vec![#stream]);))
    }
        
    quote!(alchemy::StyleSheet::new({
        use alchemy::style_attributes::*;
        use alchemy::Color;
        let mut styles = std::collections::HashMap::new();
        #body
        styles
    })).into()
}

/// Implements a derive macro for automating props setting and conversion.
#[proc_macro_derive(Props)]
pub fn writable_props_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_props = Ident::new(&format!("{}Props", name), Span::call_site());
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            fn default_props() -> #name_props {
                #name_props::default()
            }
        }

        impl #impl_generics alchemy::ComponentProps for #name #ty_generics #where_clause {
            fn set_props(&mut self, new_props: &mut Any) {
                match new_props.downcast_ref::<#name_props>() {
                    Some(props) => { },
                    None => { panic!("Woah there, somehow the wrong props were being passed!"); }
                }
            }
        }
    })
}
