use std::num::ParseFloatError;

use crate::entity::{risp_env::RispEnv, risp_err::RispErr, risp_exp::RispExp};

pub fn tokenize(expr: &str) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

// parseとread_seqが相互に依存しているためあまりよくない
pub fn parse(tokens: &[String]) -> Result<(RispExp, &[String]), RispErr> {
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

fn read_seq(tokens: &[String]) -> Result<(RispExp, &[String]), RispErr> {
    let mut res = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs.split_first().ok_or(RispErr::Reason(
            "would not find closing bracket. `)`".to_string(),
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

fn eval(exp: &RispExp, env: &RispEnv) -> Result<RispExp, RispErr> {
    match exp {
        RispExp::Symbol(symbol) => env
            .get(symbol)
            .ok_or(RispErr::Reason(format!("unexpected symbol, {}", symbol)))
            .map(|x| x.clone()),
        RispExp::Number(_) => Ok(exp.clone()),
        RispExp::List(list) => {
            let first_form = list
                .first()
                .ok_or(RispErr::Reason("unexpected empty list".to_string()))?;
            let arg_forms = &list[1..];
            let first_eval = eval(first_form, env)?;

            // function 1 2 3のようなformat(functionは例えばdefault_env()で登録した和算関数)のListで最初が関数でない場合はinvalid formatであるというイメージ
            // 仮にfunction 1 + 2だった場合はparse_float_argsでErr(RispErr)を返す
            match first_eval {
                RispExp::Func(f) => {
                    let args_eval = arg_forms
                        .iter()
                        .map(|x| eval(x, env))
                        .collect::<Result<Vec<RispExp>, RispErr>>();
                    let risp_exps = &args_eval?;
                    f(risp_exps)
                }
                _ => Err(RispErr::Reason("first form must be a function".to_string())),
            }
        }
        RispExp::Func(_) => Err(RispErr::Reason("unexpected form".to_string())),
    }
}

pub fn parse_eval(expr: String, env: &RispEnv) -> Result<RispExp, RispErr> {
    let (parsed_exp, _) = parse(&tokenize(&expr))?;
    Ok(eval(&parsed_exp, env)?)
}
