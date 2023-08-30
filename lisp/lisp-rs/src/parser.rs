use anyhow::{Result, bail};

use crate::{lisp_expr::LispExpr, token::Token};

pub fn parse(tokens: Vec<Token>) -> Result<LispExpr> {
    if tokens.is_empty() {
        bail!("Empty tokens")
    }

    let mut token_iter = tokens.into_iter().peekable();
    parse_list(&mut token_iter)
}

fn parse_list(token_iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Result<LispExpr> {
    let head = token_iter.next().unwrap();
    match head {
        Token::LParen => (),
        _ => bail!("Expression must start with left parenthesis")
    }

    let mut lisp_exprs = Vec::new();
    while let Some(token) = token_iter.peek() {
        match token {
            Token::Integer(n) => lisp_exprs.push(LispExpr::Integer(*n)),
            Token::Symbol(s) => lisp_exprs.push(LispExpr::Symbol(s.clone())),
            Token::LParen => lisp_exprs.push(parse_list(token_iter)?),
            Token::RParen => return Ok(LispExpr::List(lisp_exprs)),
        }
        token_iter.next();
    }

    bail!("Right parenthesis is missing")
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::*;

    #[test]
    fn test_parse_fail_with_empty_tokens() {
        let tokens = Vec::new();
        let lisp_expr = parse(tokens);
        assert!(lisp_expr.is_err());
    }

    #[test]
    fn test_parse_fail_with_invalid_expression() {
        let expr = "(+ 1 1";
        let lisp_expr = parse(tokenize(expr).unwrap());
        assert!(lisp_expr.is_err());
    }

    #[test]
    fn test_parse() {
        let expr = "(
                         (define r 10)
                         (define pi 314)
                         (* pi (* r r))
                       )";
        let lisp_expr = parse(tokenize(expr).unwrap()).unwrap();
        assert_eq!(
            lisp_expr,
            LispExpr::List(vec![
                LispExpr::List(vec![
                    LispExpr::Symbol("define".to_string()),
                    LispExpr::Symbol("r".to_string()),
                    LispExpr::Integer(10),
                ]),
                LispExpr::List(vec![
                    LispExpr::Symbol("define".to_string()),
                    LispExpr::Symbol("pi".to_string()),
                    LispExpr::Integer(314),
                ]),
                LispExpr::List(vec![
                    LispExpr::Symbol("*".to_string()),
                    LispExpr::Symbol("pi".to_string()),
                    LispExpr::List(vec![
                        LispExpr::Symbol("*".to_string()),
                        LispExpr::Symbol("r".to_string()),
                        LispExpr::Symbol("r".to_string()),
                    ]),
                ]),
            ])
        );
    }
}