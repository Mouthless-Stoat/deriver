use std::str::FromStr;

use crate::Expr;

mod lex;
mod parse;

pub(crate) use lex::*;
pub(crate) use parse::*;

#[derive(Debug, PartialEq, Eq)]
pub enum LangError {
    // lexer error
    MultiLine,
    InvalidFloatFormat(usize),
    InvalidSymbol(char, usize),
    InvalidWord(String, usize),

    // parser error
    UncloseParen(usize),
    UnexpectedToken(usize),
}

type Res<T> = Result<T, LangError>;

impl FromStr for Expr {
    type Err = LangError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(lex(s)?)
    }
}
