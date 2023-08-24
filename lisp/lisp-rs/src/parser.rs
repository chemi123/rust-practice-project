use core::fmt;
use std::error::Error;

use anyhow::{Result, anyhow};

use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Integer(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::Lambda(params, body) => {
                write!(f, "Lambda(")?;
                for param in params {
                    write!(f, "{} ", param)?;
                }
                write!(f, ")")?;
                for expr in body {
                    write!(f, " {}", expr)?;
                }
                Ok(())
            }
            Object::List(list) => {
                write!(f, "(")?;
                for (i, object) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", object)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    err: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

impl Error for ParseError {}

impl ParseError {
    pub fn new(message: &str) -> Self {
        ParseError {
            err: message.to_string(),
        }
    }
}

pub fn parse_tokens(tokens: &mut Vec<Token>) -> Result<Object> {
    tokens.reverse();
    let first_token = tokens.pop().ok_or(ParseError::new("empty tokens"))?;
    if first_token != Token::LParen {
        return Err(anyhow!("tokens must start with left parenthesis"))
    }

    parse_tokens_inner(tokens)
}

fn parse_tokens_inner(tokens: &mut Vec<Token>) -> Result<Object> {
    let mut objects: Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop().unwrap();
        match token {
            Token::Integer(n) => objects.push(Object::Integer(n)),
            Token::Symbol(s) => objects.push(Object::Symbol(s)),
            Token::LParen => objects.push(parse_tokens_inner(tokens)?),
            Token::RParen => return Ok(Object::List(objects)),
        }
    }

    // ここに来る時点で最後のトークンが")"になっていないため不正
    Err(anyhow!("given an invalid expression"))
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::*;

    #[test]
    fn test_parse() {}

    #[test]
    fn test_area_of_a_circle() {
        let expr = "(
                         (define r 10)
                         (define pi 314)
                         (* pi (* r r))
                       )";
        let objects = parse_tokens(&mut tokenize(expr).unwrap()).unwrap();
        assert_eq!(
            objects,
            Object::List(vec![
                Object::List(vec![
                    Object::Symbol("define".to_string()),
                    Object::Symbol("r".to_string()),
                    Object::Integer(10),
                ]),
                Object::List(vec![
                    Object::Symbol("define".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::Integer(314),
                ]),
                Object::List(vec![
                    Object::Symbol("*".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::List(vec![
                        Object::Symbol("*".to_string()),
                        Object::Symbol("r".to_string()),
                        Object::Symbol("r".to_string()),
                    ]),
                ]),
            ])
        );
    }
}
