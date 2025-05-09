use std::collections::VecDeque;

use crate::{
    lang::lex::{Token, TokenType},
    Expr, Trig,
};

use super::{LangError, Res};

struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    fn parse(&mut self) -> Res<Expr> {
        self.parse_add_bin()
    }

    fn parse_add_bin(&mut self) -> Res<Expr> {
        let mut left = self.parse_mul_bin()?;
        while matches!(self.curr(), TokenType::Plus | TokenType::Minus) {
            let op = self.next();
            let right = self.parse_mul_bin()?;

            left = match op {
                TokenType::Plus => left.add(right),
                TokenType::Minus => left.sub(right),
                _ => unreachable!(),
            }
        }

        Ok(left)
    }

    fn parse_mul_bin(&mut self) -> Res<Expr> {
        let mut left = self.parse_func()?;
        while matches!(self.curr(), TokenType::Star | TokenType::Slash) {
            let op = self.next();
            let right = self.parse_func()?;

            left = match op {
                TokenType::Star => left.mul(right),
                TokenType::Slash => left.div(right),
                _ => unreachable!(),
            }
        }

        Ok(left)
    }

    fn parse_func(&mut self) -> Res<Expr> {
        Ok(match self.curr() {
            func @ (TokenType::Sin
            | TokenType::Cos
            | TokenType::Tan
            | TokenType::Csc
            | TokenType::Sec
            | TokenType::Cot) => {
                self.next();

                self.parse_exp_bin()?.trig(match func {
                    TokenType::Sin => Trig::Sin,
                    TokenType::Cos => Trig::Cos,
                    TokenType::Tan => Trig::Tan,
                    TokenType::Csc => Trig::Csc,
                    TokenType::Sec => Trig::Sec,
                    TokenType::Cot => Trig::Cot,

                    _ => unreachable!(),
                })
            }

            TokenType::Log => {
                self.next();

                let base = if matches!(self.curr(), TokenType::Underscore) {
                    self.next();
                    self.parse_exp_bin()?
                } else {
                    Expr::Num(10.0)
                };

                self.parse_exp_bin()?.log(base)
            }

            _ => self.parse_exp_bin()?,
        })
    }

    fn parse_exp_bin(&mut self) -> Res<Expr> {
        let mut left = self.parse_unit()?;
        while matches!(self.curr(), TokenType::Caret) {
            let op = self.next();
            let right = self.parse_unit()?;

            left = match op {
                TokenType::Caret => left.exp(right),
                _ => unreachable!(),
            }
        }

        Ok(left)
    }

    fn parse_unit(&mut self) -> Res<Expr> {
        let t = self.next_token();
        Ok(match t.token {
            TokenType::OpenParen => {
                let temp = self.parse();
                if self.expect(TokenType::CloseParen) {
                    self.next();
                    return temp;
                } else {
                    return Err(LangError::UncloseParen(self.next_token().loc));
                };
            }
            TokenType::Num(n) => Expr::Num(n),
            TokenType::Var => Expr::Var,

            _ => return Err(LangError::UnexpectedToken(t.loc)),
        })
    }

    fn curr(&self) -> TokenType {
        self.curr_token().token
    }

    fn curr_token(&self) -> Token {
        *self.tokens.front().unwrap()
    }

    fn next(&mut self) -> TokenType {
        self.next_token().token
    }

    fn next_token(&mut self) -> Token {
        self.tokens.pop_front().unwrap()
    }

    fn expect(&mut self, tk: TokenType) -> bool {
        self.curr() == tk
    }
}

pub(crate) fn parse(tokens: Vec<Token>) -> Res<Expr> {
    Parser {
        tokens: tokens.into(),
    }
    .parse()
}
