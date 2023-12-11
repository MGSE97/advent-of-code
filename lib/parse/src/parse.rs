use std::{collections::BTreeMap, rc::Rc};

use proc_macro2::TokenTree;
use syn::{
    braced,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseBuffer, ParseStream},
    token::Paren,
    Ident, LitStr, Result, Token, Type,
};

pub struct ParseOptions {
    pub input: Ident,
    pub exclude: LitStr,
    pub format: Vec<FormatToken>,
    pub tokens: BTreeMap<String, ParseToken>,
}

pub enum FormatToken {
    Token((Ident, bool)),
    Single(Rc<FormatToken>),
    Multiple(Rc<FormatToken>),
    Group(Vec<FormatToken>),
}

pub enum ParseToken {
    Simple(Ident, LitStr),
    Typed(Ident, Type, LitStr),
    Renamed(Ident, Ident),
}

impl ParseToken {
    pub fn name(&self) -> &Ident {
        match self {
            ParseToken::Simple(name, _) => name,
            ParseToken::Typed(name, _, _) => name,
            ParseToken::Renamed(name, _) => name,
        }
    }
}

mod token {
    use syn::custom_keyword;

    custom_keyword!(without);
}

impl Parse for ParseOptions {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        let input: Ident = stream.parse()?;

        let format_group;
        braced!(format_group in stream);

        stream.parse::<token::without>()?;

        let exclude: LitStr = stream.parse()?;

        stream.parse::<Token![as]>()?;

        let token_group;
        braced!(token_group in stream);

        // parse tokens
        let mut tokens = BTreeMap::new();
        while let Ok(token) = token_group.parse::<ParseToken>() {
            tokens.insert(token.name().to_string(), token);
        }

        // parse format
        let format = parse_group(&tokens, format_group)?;

        Ok(Self {
            input,
            exclude,
            format,
            tokens,
        })
    }
}

impl Parse for ParseToken {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let mut inner_type: Option<Type> = None;
        if !input.peek(LitStr) {
            inner_type = Some(input.parse()?);
        }

        let mut rename: Option<Ident> = None;
        let mut matcher: Option<LitStr> = None;
        if input.peek(Token![use]) {
            input.parse::<Token![use]>()?;
            rename = Some(input.parse()?);
        } else {
            matcher = Some(input.parse()?);
        }

        match (inner_type, rename, matcher) {
            (None, None, Some(matcher)) => Ok(ParseToken::Simple(name, matcher)),
            (None, Some(rename), None) => Ok(ParseToken::Renamed(name, rename)),
            (Some(inner_type), None, Some(matcher)) => {
                Ok(ParseToken::Typed(name, inner_type, matcher))
            }
            _ => Err(syn::Error::new(name.span(), "Wrong token definition!")),
        }
    }
}

fn parse_group(
    tokens: &BTreeMap<String, ParseToken>,
    group: ParseBuffer<'_>,
) -> Result<Vec<FormatToken>> {
    let mut format = Vec::new();
    while !group.is_empty() {
        if group.peek(Ident::peek_any) {
            let ident: Ident = group.parse()?;
            let plus = group.peek(Token![+]);
            if let Some(found) = tokens.get(&ident.to_string()) {
                let token = Rc::new(FormatToken::Token((
                    ident,
                    matches!(found, ParseToken::Typed(_, _, _)),
                )));
                format.push(match plus {
                    false => FormatToken::Single(token),
                    true => FormatToken::Multiple(token),
                });
            }
        } else if group.peek(Paren) {
            let inner;
            parenthesized!(inner in group);

            let parsed = Rc::new(FormatToken::Group(parse_group(tokens, inner)?));
            format.push(match group.peek(Token![+]) {
                false => FormatToken::Single(parsed),
                true => FormatToken::Multiple(parsed),
            });
        } else {
            // Skip tokens we don't care about
            let _ = group.parse::<TokenTree>();
        }
    }
    Ok(format)
}
