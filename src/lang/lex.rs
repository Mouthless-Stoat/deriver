use super::{LangError, Res};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    OpenParen,
    CloseParen,

    Num(f64),
    Var,

    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Underscore,

    Log,
    Sin,
    Cos,
    Tan,
    Csc,
    Sec,
    Cot,

    #[allow(clippy::upper_case_acronyms)]
    END,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Token {
    pub token: TokenType,
    pub loc: usize,
}

impl Token {
    pub fn end() -> Self {
        Self {
            token: TokenType::END,
            loc: usize::MAX,
        }
    }
}

impl TokenType {
    pub fn at(self, loc: usize) -> Token {
        Token { token: self, loc }
    }
}

pub(crate) fn lex(source: &str) -> Res<Vec<Token>> {
    if source.lines().count() > 1 {
        return Err(LangError::MultiLine);
    }

    let mut tokens = vec![];

    let mut src = source.char_indices().peekable();

    while let Some((loc, char)) = src.next() {
        if char.is_whitespace() {
            continue;
        }

        if !char.is_alphanumeric() {
            let tk_opt = match char {
                '+' => Some(TokenType::Plus),
                '-' => Some(TokenType::Minus),
                '*' => Some(TokenType::Star),
                '/' => Some(TokenType::Slash),
                '^' => Some(TokenType::Caret),
                '_' => Some(TokenType::Underscore),
                '(' => Some(TokenType::OpenParen),
                ')' => Some(TokenType::CloseParen),
                _ => None,
            };

            if let Some(token) = tk_opt {
                tokens.push(token.at(loc));
                continue;
            } else {
                return Err(LangError::InvalidSymbol(char, loc));
            }
        }

        let is_num = char.is_numeric();
        let mut acc = String::from(char);

        while let Some((_, c)) = src.next_if(|&(_, c)| {
            if is_num {
                c.is_numeric() || c == '.' || c == ','
            } else {
                c.is_alphabetic()
            }
        }) {
            acc.push(c);
        }

        if is_num {
            if acc.chars().filter(|&c| c == '.' || c == ',').count() <= 1 {
                tokens.push(TokenType::Num(acc.parse().unwrap()).at(loc));
                continue;
            }

            return Err(LangError::InvalidFloatFormat(loc));
        } else {
            let word_opt = match acc.as_str() {
                "log" => Some(TokenType::Log),

                "sin" => Some(TokenType::Sin),
                "cos" => Some(TokenType::Cos),
                "tan" => Some(TokenType::Tan),
                "csc" => Some(TokenType::Csc),
                "sec" => Some(TokenType::Sec),
                "cot" => Some(TokenType::Cot),

                "x" => Some(TokenType::Var),

                _ => None,
            };

            if let Some(token) = word_opt {
                tokens.push(token.at(loc));
            } else {
                return Err(LangError::InvalidWord(acc, loc));
            }
        }
    }

    tokens.push(Token::end());

    Ok(tokens)
}
