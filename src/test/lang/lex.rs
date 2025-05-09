use crate::lang::{lex, Token, TokenType};

#[test]
fn simple() {
    let src = "1 + 2 * 3";
    assert_eq!(
        lex(src).unwrap(),
        vec![
            TokenType::Num(1.0).at(0),
            TokenType::Plus.at(2),
            TokenType::Num(2.0).at(4),
            TokenType::Star.at(6),
            TokenType::Num(3.0).at(8),
            Token::end()
        ]
    )
}

#[test]
fn float() {
    let src = "0.4 - 0.1";
    assert_eq!(
        lex(src).unwrap(),
        vec![
            TokenType::Num(0.4).at(0),
            TokenType::Minus.at(4),
            TokenType::Num(0.1).at(6),
            Token::end()
        ]
    )
}

#[test]
fn function() {
    let src = "log sin cos tan";
    assert_eq!(
        lex(src).unwrap(),
        vec![
            TokenType::Log.at(0),
            TokenType::Sin.at(4),
            TokenType::Cos.at(8),
            TokenType::Tan.at(12),
            Token::end()
        ]
    )
}

#[test]
fn symbol() {
    let src = "+-/*()";
    assert_eq!(
        lex(src).unwrap(),
        vec![
            TokenType::Plus.at(0),
            TokenType::Minus.at(1),
            TokenType::Slash.at(2),
            TokenType::Star.at(3),
            TokenType::OpenParen.at(4),
            TokenType::CloseParen.at(5),
            Token::end()
        ]
    )
}

mod error {
    use crate::lang::{lex, LangError};

    #[test]
    fn multiline() {
        let src = "1\n2";
        assert_eq!(lex(src), Err(LangError::MultiLine))
    }

    #[test]
    fn invalid_float() {
        let src = "1.1.1.2";
        assert_eq!(lex(src), Err(LangError::InvalidFloatFormat(0)))
    }

    #[test]
    fn invalid_symbol() {
        let src = "\"";
        assert_eq!(lex(src), Err(LangError::InvalidSymbol('"', 0)))
    }
}
