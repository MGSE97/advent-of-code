use std::{mem::discriminant, str::FromStr};

use logos::{Lexer, Logos};

pub fn parse<'l, Output, Token>(lex: &mut Lexer<'l, Token>) -> Option<Output>
where
    Token: Logos<'l, Source = str>,
    Output: FromStr,
{
    lex.slice().parse().ok()
}

pub fn get_token<'l, Token>(
    lex: &mut Lexer<'l, Token>,
    variant: Token,
    err: &'l str,
) -> Result<Token, &'l str>
where
    Token: Logos<'l, Source = str>,
{
    let token = lex.next();
    match token {
        Some(Ok(val)) if discriminant(&val) == discriminant(&variant) => Ok(val),
        _ => Err(err),
    }
}

pub trait LexerExt<'l, Token>
where
    Token: Logos<'l, Source = str>,
{
    fn get_token(
        &mut self,
        buff: &mut Option<Token>,
        variant: Token,
        err: &'l str,
    ) -> Result<Token, &'l str>;
}

impl<'l, Token> LexerExt<'l, Token> for Lexer<'l, Token>
where
    Token: Logos<'l, Source = str>,
{
    fn get_token(
        &mut self,
        buff: &mut Option<Token>,
        variant: Token,
        err: &'l str,
    ) -> Result<Token, &'l str> {
        match buff.take().or_else(|| self.next().and_then(|r| r.ok())) {
            Some(val) => match discriminant(&val) == discriminant(&variant) {
                true => Ok(val),
                false => {
                    let _ = buff.insert(val);
                    Err(err)
                }
            },
            None => Err(err),
        }
    }
}
