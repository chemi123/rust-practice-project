use std::{collections::VecDeque, num::ParseFloatError};

use crate::entity::{risp_err::RispErr, risp_exp::RispExp};

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

    // &tokenで&Stringになる. &[]はスライス(&str)になるため, &token[..]はtokenのスライスの全体を表すことになる
    match &token[..] {
        "(" => read_seq2(tokens),
        ")" => Err(RispErr::Reason(
            "unexpected closing bracket. `)`".to_string(),
        )),
        _ => Err(RispErr::Reason("unexpected token".to_string())),
    }
}

pub fn read_seq2(mut tokens: VecDeque<String>) -> Result<(RispExp, VecDeque<String>), RispErr> {
    let mut res = Vec::new();

    while !tokens.is_empty() {
        // !token.is_empty()の時だけループが実行されるため, tokensが空でpanicしてクラッシュすることはない
        let token = tokens.pop_front().unwrap();
        if token == ")" {
            return Ok((RispExp::List(res), tokens));
        }
        res.push(parse_atom2(token));
    }

    // "( + 1"などのように閉じ括弧がないケースはここに来る
    Err(RispErr::Reason("invalid expression".to_string()))
}

fn parse_atom2(token: String) -> RispExp {
    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(v) => RispExp::Number(v),
        Err(_) => RispExp::Symbol(token),
    }
}
