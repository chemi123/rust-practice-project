use anyhow::{Result, bail};

use crate::token::Token;

pub fn tokenize(expr_str: &str) -> Result<Vec<Token>> {
    if expr_str.trim().is_empty() {
        bail!("Empty expression");
    }

    let characters = expr_str.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|token| token.to_string())
        .collect::<Vec<_>>();

    let tokens = characters.iter()
        .map(|character| {
            match character.as_str() {
                "(" => Token::LParen,
                ")" => Token::RParen,
                _ => {
                    if let Some(n) = character.parse::<i64>().ok() {
                        Token::Integer(n)
                    } else {
                        Token::Symbol(character.clone())
                    }
                }
            }
        }).collect::<Vec<_>>();
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_tokenize_fail() {
    //     let expr = "+ 1 1";
    //     let tokens = tokenize(expr);
    //     assert!(tokens.is_err());
    // }

    #[test]
    fn test_tokenize() {
        let expr = "
        (
            (define r 10)
            (define pi 314)
            (* pi (* r r))
        )
        ";
        let tokens = tokenize(expr).unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("r".to_string()),
                Token::Integer(10),
                Token::RParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("pi".to_string()),
                Token::Integer(314),
                Token::RParen,
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("pi".to_string()),
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("r".to_string()),
                Token::Symbol("r".to_string()),
                Token::RParen,
                Token::RParen,
                Token::RParen,
            ]
        );
    }
}
