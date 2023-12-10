extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::parse::ParseOptions;

mod format;
mod parse;

/**
   ### Creates enum, that parses input into tokens in exact order.

   It will create enum for `Tokens` using `Logos` crate.
   Then it will add `parse` method, that will iterate through tokens
   and return them in exactly same order.
   If some token is not found, parsing fails.

   ### Syntax:
   ```ignore
   let tokens = parse! {
        enum_name {
            Input format
        }
        without "regex for skiping characters in input"
        as {
            Token "token text representation"
            WithData Type "regex for matching type characters"
            ToDo: Renamed use WithData // Parse token `Renamed` same way as token `WithData`
            ...
        }
   };
   ```
   ### Input formats:
    - `Token` - parse 1 token
    - `(Token)+` - parse N tokens, until some other token is found
    - ToDo: `Token|Other` - parse any of specified tokens

   Other characters are ignored.

   ### Example:
   ```ignore
   let (token, token, ...) parse! {
        input {
            This is an input file.
            User inputs: (Text)+
            Input-to-Ouptut map:
            (
                Text  Number  Text|Number
            )+
        }
        without "[:\s]+"
        as {
            InputFile "This is an input file."
            UserInputs "User inputs"
            Text String "/[a-zA-Z]+/"
            Number u32 "/[0+9]+/"
            Input use Text
            Output use Text
        }
   };
   ```
*/
#[proc_macro]
pub fn impl_parse(stream: TokenStream) -> TokenStream {
    let options = &parse_macro_input!(stream as ParseOptions);

    let input = &options.input;
    let skip = &options.exclude;
    let tokens = options.format_tokens();
    let mapper = options.format_mapper();
    let result = options.format_result();

    quote! {
        use ::lib::LexerExt;
        use ::std::default::Default;
        #[derive(::logos::Logos, Debug, PartialEq)]
        #[logos(skip #skip)]
        pub enum #input {
            #tokens
        }
        impl #input {
            pub fn parse(input: &str) -> Result<(#result), String> {
                let mut tokens = <Self as ::logos::Logos>::lexer(input);
                let mut buff: Option<Self> = None;
                Ok((#mapper))
            }
        }
    }
    .into()
}
