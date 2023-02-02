use std::num::ParseFloatError;

use crate::entity::{RispEnv, RispErr, RispExpr};

pub fn tokenize(expr: &str) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

// parseとread_seqが相互に依存しているためあまりよくない
pub fn parse(tokens: &[String]) -> Result<(RispExpr, &[String]), RispErr> {
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

fn read_seq(tokens: &[String]) -> Result<(RispExpr, &[String]), RispErr> {
    let mut res = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs.split_first().ok_or(RispErr::Reason(
            "would not find closing bracket. `)`".to_string(),
        ))?;
        if next_token == ")" {
            return Ok((RispExpr::List(res), rest));
        }

        let (expr, new_xs) = parse(&xs)?;
        res.push(expr);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> RispExpr {
    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(v) => RispExpr::Number(v),
        Err(_) => RispExpr::Symbol(token.to_string()),
    }
}

fn eval(expr: &RispExpr, env: &RispEnv) -> Result<RispExpr, RispErr> {
    match expr {
        RispExpr::Symbol(symbol) => env
            .get(symbol)
            .ok_or(RispErr::Reason(format!("unexpected symbol, {}", symbol)))
            .map(|x| x.clone()),
        RispExpr::Number(_) => Ok(expr.clone()),
        RispExpr::List(list) => {
            let first_form = list
                .first()
                .ok_or(RispErr::Reason("unexpected empty list".to_string()))?;
            let arg_forms = &list[1..];
            let first_eval = eval(first_form, env)?;

            // function 1 2 3のようなformat(functionは例えばdefault_env()で登録した和算関数)のListで最初が関数でない場合はinvalid formatであるというイメージ
            // 仮にfunction 1 + 2だった場合はparse_float_argsでErr(RispErr)を返す
            match first_eval {
                RispExpr::Func(f) => {
                    let args_eval = arg_forms
                        .iter()
                        .map(|x| eval(x, env))
                        .collect::<Result<Vec<RispExpr>, RispErr>>();
                    let risp_exprs = &args_eval?;
                    f(risp_exprs)
                }
                _ => Err(RispErr::Reason("first form must be a function".to_string())),
            }
        }
        RispExpr::Func(_) => Err(RispErr::Reason("unexpected form".to_string())),
    }
}

pub fn parse_risp_expr_string(expr: String, env: &RispEnv) -> Result<RispExpr, RispErr> {
    let (parsed_expr, _) = parse(&tokenize(&expr))?;
    Ok(eval(&parsed_expr, env)?)
}
