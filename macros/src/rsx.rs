use proc_macro2::{Delimiter, Group, Ident, Literal, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};

use crate::error::ParseError;
use crate::ident;
use crate::lexer::{/*to_stream, */Lexer, Token};
use crate::map::StringyMap;
use crate::parser::grammar;

use std::iter::FromIterator;

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Text(Literal),
    Block(Group),
}

impl Node {
    pub fn into_token_stream(self, ty: &Option<Vec<Token>>) -> Result<TokenStream, TokenStream> {
        match self {
            Node::Element(el) => el.into_token_stream(ty),
            Node::Text(text) => {
                let text = TokenTree::Literal(text);
                Ok(quote!(alchemy::RSX::text(#text.to_string())))
            }
            Node::Block(group) => {
                let span = group.span();
                let error =
                    "you cannot use a block as a top level element or a required child element";
                Err(quote_spanned! { span=>
                    compile_error! { #error }
                })
            }
        }
    }

    fn into_child_stream(self, ty: &Option<Vec<Token>>) -> Result<TokenStream, TokenStream> {
        match self {
            Node::Element(el) => {
                let el = el.into_token_stream(ty)?;
                Ok(quote!(
                    /*element.*/children.push(#el);
                ))
            }
            tx @ Node::Text(_) => {
                let tx = tx.into_token_stream(ty)?;
                Ok(quote!(
                    /*element.*/children.push(#tx);
                ))
            }
            Node::Block(group) => {
                let group: TokenTree = group.into();
                Ok(quote!(
                    for child in #group.into_iter() {
                        /*element.*/children.push(child);
                    }
                ))
            }
        }
    }
}

#[derive(Clone)]
pub struct Element {
    pub name: Ident,
    pub attributes: StringyMap<Ident, TokenTree>,
    pub children: Vec<Node>,
}

fn extract_event_handlers(
    attrs: &mut StringyMap<Ident, TokenTree>,
) -> StringyMap<Ident, TokenTree> {
    let mut events = StringyMap::new();
    let keys: Vec<Ident> = attrs.keys().cloned().collect();
    for key in keys {
        let key_name = key.to_string();
        let prefix = "on";
        if key_name.starts_with(prefix) {
            let event_name = &key_name[prefix.len()..];
            let value = attrs.remove(&key).unwrap();
            events.insert(ident::new_raw(event_name, key.span()), value);
        }
    }
    events
}

fn process_value(value: &TokenTree) -> TokenStream {
    match value {
        TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket => {
            let content = g.stream();
            quote!( [ #content ] )
        }
        TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => {
            let content = g.stream();
            quote!( ( #content ) )
        }
        v => TokenStream::from_iter(vec![v.clone()]),
    }
}

fn is_string_literal(literal: &Literal) -> bool {
    // This is the worst API
    literal.to_string().starts_with('"')
}

#[allow(dead_code)]
fn stringify_ident(ident: &Ident) -> String {
    let s = ident.to_string();
    if s.starts_with("r#") {
        s[2..].to_string()
    } else {
        s
    }
}

impl Element {
    fn into_token_stream(mut self, ty: &Option<Vec<Token>>) -> Result<TokenStream, TokenStream> {
        let name = self.name;
        let name_str = name.to_string();
        let typename: TokenTree = Ident::new(&name_str, name.span()).into();
        
        let events = extract_event_handlers(&mut self.attributes);
        let attrs = self.attributes.iter().map(|(key, value)| {
            let name = key.to_string();
            let token = TokenTree::Ident(ident::new_raw(&name, key.span()));
            (name, token, value)
        });
        
       
        let mut attributes = TokenStream::new();
        let mut styles = TokenStream::new();
        styles.extend(quote!(alchemy::SpacedSet::new()));

        for (attr_str, key, value) in attrs {
            match value {
                TokenTree::Literal(lit) if is_string_literal(lit) => {
                    let mut eprintln_msg = "ERROR: ".to_owned();
                    #[cfg(can_show_location_of_runtime_parse_error)]
                    {
                        let span = lit.span();
                        eprintln_msg += &format!(
                            "{}:{}:{}: ",
                            span.unstable()
                                .source_file()
                                .path()
                                .to_str()
                                .unwrap_or("unknown"),
                            span.unstable().start().line,
                            span.unstable().start().column
                        );
                    }
                    eprintln_msg += &format!(
                        "<{} {}={}> failed to parse attribute value: {{}}",
                        name_str, attr_str, lit,
                    );
                    #[cfg(not(can_show_location_of_runtime_parse_error))]
                    {
                        eprintln_msg += "\nERROR: rebuild with nightly to print source location";
                    }

                    //body.extend(quote!(
                        /*element.attrs.#key = Some(#lit.parse().unwrap_or_else(|err| {
                            eprintln!(#eprintln_msg, err);
                            panic!("failed to parse string literal");
                        }));*/
                    //));
                },

                value => {
                    let key = key.to_string();
                    let value = process_value(value);
                    
                    if key == "r#styles" {
                        styles = quote!(std::convert::Into::into(#value));
                        continue;
                    }

                    if key == "r#key" {
                        continue;
                    }

                    attributes.extend(quote!(
                        attributes.insert(#key, std::convert::Into::into(#value));
                    ));
                }
            }
        }
        
        for (key, _value) in events.iter() {
            if ty.is_none() {
                let mut err = quote_spanned! { key.span() =>
                    compile_error! { "when using event handlers, you must declare the output type inside the rsx! macro" }
                };
                let hint = quote_spanned! { Span::call_site() =>
                    compile_error! { "for example: change rsx!(<div>...</div>) to rsx!(<div>...</div> : String)" }
                };
                err.extend(hint);
                return Err(err);
            }
            //let key = TokenTree::Ident(key.clone());
            //let value = process_value(value);
            /*body.extend(quote!(
                element.events.#key = Some(alchemy::dom::events::IntoEventHandler::into_event_handler(#value));
            ));*/
        }

        /*let mut args = TokenStream::new();
        let mut type_annotation = TokenStream::new();
        if let Some(ty) = ty {
            let type_var = to_stream(ty.clone());
            type_annotation.extend(quote!(: #typename<#type_var>));
        }*/
 
        let mut children = TokenStream::new();
        children.extend(self.children.into_iter().map(|node| {
            node.into_child_stream(ty)
        }).collect::<Result<Vec<TokenStream>, TokenStream>>()?);

        let component_name = Literal::string(&typename.to_string());

        Ok(quote!(
            alchemy::RSX::node(#component_name, |key| {
                Box::new(#typename::constructor(key))
            }, alchemy::Props::new("".into(), #styles, {
                let mut attributes = std::collections::HashMap::new();
                #attributes
                attributes
            }), {
                let mut children = vec![];
                #children
                children
            })
        ))
    }
}

// FIXME report a decent error when the macro contains multiple top level elements
pub fn expand_rsx(input: &[Token]) -> Result<(Node, Option<Vec<Token>>), ParseError> {
    grammar::NodeWithTypeParser::new().parse(Lexer::new(input))
}
