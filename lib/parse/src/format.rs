use std::ops::Deref;

use crate::parse::{FormatToken, ParseOptions, ParseToken};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{punctuated::Punctuated, LitStr, Token};

pub fn map_format_token(token: &FormatToken, can_throw: bool) -> TokenStream {
    match token {
        FormatToken::Token((token, typed)) => {
            let matcher = match typed {
                true => quote!(#token(Default::default())),
                false => quote!(#token),
            };
            let err = LitStr::new(&token.to_string(), token.span());
            quote_spanned! {
                token.span() =>
                tokens.get_token(Self::#matcher, #err)
            }
        }
        FormatToken::Single(single) => {
            let mapper = map_format_token(single, can_throw);
            match can_throw && !matches!(single.deref(), FormatToken::Group(..)) {
                true => quote!(#mapper?),
                false => quote!(#mapper),
            }
        }
        FormatToken::Multiple(multiple) => {
            let mapper = map_format_token(multiple, false);
            quote! {
                {
                    let mut items = Vec::new();
                    while let Ok(token) = #mapper {
                        items.push(token);
                    }
                    items
                }
            }
        }
        FormatToken::Group(group) => {
            let items: Punctuated<_, Token![,]> = group
                .iter()
                .map(|item| map_format_token(item, can_throw))
                .collect();
            match (items.len(), can_throw) {
                (1, _) => items[0].to_owned(),
                (_, true) => quote!((#items)),
                (_, false) => {
                    let mut iter = items.iter();
                    let first = iter.next().unwrap().to_owned();
                    let rest: Punctuated<_, Token![.]> = iter
                        .take(items.len() - 1)
                        .map(|item| {
                            quote! {
                                and_then(|prev| {
                                    match #item {
                                        Ok(val) => Ok((prev, val))
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
        self.tokens
            .values()
            .map(|token| match token {
                ParseToken::Simple(name, matcher) => quote_spanned! {
                    name.span() =>
                    #[token(#matcher)] #name
                },
                ParseToken::Typed(name, inner, matcher) => quote_spanned! {
                    name.span() =>
                    #[regex(#matcher, ::lib::parse)]
                    #name(#inner)
                },
                ParseToken::Renamed(name, matcher) => todo!(),
            })
            .collect()
    }

    pub fn format_mapper(&self) -> Punctuated<TokenStream, Token![,]> {
        self.format
            .iter()
            .map(|item| map_format_token(item, true))
            .collect()
    }

    pub fn format_result(&self) -> Punctuated<TokenStream, Token![,]> {
        self.format.iter().map(map_format_token_types).collect()
    }
}
