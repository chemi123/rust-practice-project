use std::{collections::HashMap, num::ParseFloatError};

use crate::entity::{risp_env::RispEnv, risp_err::RispErr, risp_exp::RispExp};

pub fn tokenize(expr: &str) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

// parseとread_seqが相互に依存しているためあまりよくない
pub fn parse<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(RispErr::Reason("could not get tokens".to_string()))?;

    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(RispErr::Reason(
            "unexpected closing bracket. `)`".to_string(),
        )),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn read_seq<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
    let mut res = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs.split_first().ok_or(RispErr::Reason(
            "ould not find closing bracket. `)`".to_string(),
        ))?;
        if next_token == ")" {
            return Ok((RispExp::List(res), rest));
        }

        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> RispExp {
    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(v) => RispExp::Number(v),
        Err(_) => RispExp::Symbol(token.to_string()),
    }
}
