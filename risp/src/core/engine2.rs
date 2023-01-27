use std::{collections::VecDeque, num::ParseFloatError};

use crate::entity::{risp_err::RispErr, risp_exp::RispExp};

// originalのengineの方は関数が相互依存しているなど気になる点があった. こっちではそれを修正してみる

pub fn tokenize2(expr: &str) -> VecDeque<String> {
    expr.replace("(", "( ")
        .replace(")", " )")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

pub fn parse2(mut tokens: VecDeque<String>) -> Result<(RispExp, VecDeque<String>), RispErr> {
    let token = tokens
        .pop_front()
        .ok_or(RispErr::Reason("tokens is empty".to_string()))?;
    let mut res = Vec::new();

    // &tokenで&Stringになる. &[]はスライス(&str)になるため, &token[..]はtokenのスライスの全体を表すことになる
    match &token[..] {
        "(" => (),
        ")" => return Err(RispErr::Reason("unexpected closing bracket".to_string())),
        _ => res.push(parse_atom(&token)),
    };

    while !tokens.is_empty() {
        let token = tokens.pop_front().unwrap();
        match token.as_str() {
            "(" => {
                let (risp_exp, ret_tokens) = parse2(tokens)?;
                tokens = ret_tokens;
                res.push(risp_exp);
            }
            ")" => return Ok((RispExp::List(res), tokens)),
            _ => res.push(parse_atom(&token)),
        };
    }

    // "( + 1"などのように閉じ括弧がないケースはここに来る
    Err(RispErr::Reason("invalid expression".to_string()))
}

fn parse_atom(token: &str) -> RispExp {
    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(v) => RispExp::Number(v),
        Err(_) => RispExp::Symbol(token.to_string()),
    }
}
