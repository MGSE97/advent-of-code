use std::ops::Deref;

use crate::parse::{FormatToken, ParseOptions, ParseToken};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{punctuated::Punctuated, LitInt, LitStr, Token};

pub fn map_format_token(token: &FormatToken, needs_result: bool) -> TokenStream {
    match token {
        FormatToken::Token((token, typed)) => {
            let matcher = match typed {
                true => quote!(#token(Default::default())),
                false => quote!(#token),
            };
            let err = LitStr::new(&token.to_string(), token.span());
            quote_spanned! {
                token.span() =>
                tokens.get_token(&mut buff, Self::#matcher, #err)
            }
        }
        FormatToken::Single(single) => {
            let mapper = map_format_token(single, needs_result);
            match needs_result || matches!(single.deref(), FormatToken::Group(..)) {
                false => quote!(#mapper?),
                true => quote!(#mapper),
            }
        }
        FormatToken::Multiple(multiple) => {
            let mapper = map_format_token(multiple, true);
            let result = match needs_result {
                true => quote! {
                    match items.is_empty() {
                        true => Err("No values!"),
                        false => Ok(items)
                    }
                },
                false => quote!(items),
            };
            quote! {
                {
                    let mut items = Vec::new();
                    while let Ok(token) = #mapper {
                        items.push(token);
                    }
                    #result
                }
            }
        }
        FormatToken::Group(group) => {
            let items: Punctuated<_, Token![,]> = group
                .iter()
                .map(|item| map_format_token(item, needs_result))
                .collect();
            match (items.len(), needs_result) {
                (1, _) => items[0].to_owned(),
                (_, false) => quote!((#items)),
                (_, true) => {
                    let mut iter = items.iter();
                    let first = iter.next().unwrap().to_owned();
                    let rest: Punctuated<_, Token![.]> = iter
                        .take(items.len() - 1)
                        .map(|item| {
                            quote! {
                                and_then(|prev| {
                                    match #item {
                                        Ok(val) => Ok((prev, val)),
                                        Err(err) => Err(err)
                                    }
                                })
                            }
                        })
                        .collect();

                    quote! {
                        #first
                        .#rest
                    }
                }
            }
        }
    }
}

pub fn map_format_token_types(token: &FormatToken) -> TokenStream {
    match token {
        FormatToken::Token((token, _)) => {
            quote_spanned! {
                token.span() =>
                Self
            }
        }
        FormatToken::Single(single) => map_format_token_types(single),
        FormatToken::Multiple(multiple) => {
            let inner_type = map_format_token_types(multiple);
            quote!(Vec::<#inner_type>)
        }
        FormatToken::Group(group) => {
            let items: Punctuated<_, Token![,]> =
                group.iter().map(map_format_token_types).collect();
            match items.len() {
                1 => items[0].to_owned(),
                _ => quote!((#items)),
            }
        }
    }
}

impl ParseOptions {
    pub fn format_tokens(&self) -> Punctuated<TokenStream, Token![,]> {
        let max_priority = self.tokens.len() + 1;
        self.tokens
            .values()
            .enumerate()
            .map(|(i, token)| {
                let priority = LitInt::new(&(max_priority - i).to_string(), token.name().span());
                match token {
                    ParseToken::Simple(name, matcher) => quote_spanned! {
                        name.span() =>
                        #[token(#matcher, priority=#priority)] #name
                    },
                    ParseToken::Typed(name, inner, matcher) => quote_spanned! {
                        name.span() =>
                        #[regex(#matcher, ::lib::parse, priority=#priority)]
                        #name(#inner)
                    },
                    ParseToken::Renamed(_name, _matcher) => todo!(),
                }
            })
            .collect()
    }

    pub fn format_mapper(&self) -> Punctuated<TokenStream, Token![,]> {
        self.format
            .iter()
            .map(|item| map_format_token(item, false))
            .collect()
    }

    pub fn format_result(&self) -> Punctuated<TokenStream, Token![,]> {
        self.format.iter().map(map_format_token_types).collect()
    }
}
