use crate::{
    entity::{RispEnv, RispErr, RispExpr},
    utils::parse_atom,
};

// parseとread_seqが相互に依存しているためあまりよくない
pub fn parse_risp_exprs(tokens: &[String]) -> Result<(RispExpr, &[String]), RispErr> {
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

pub fn parse_risp_expr_string(expr: String, env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    let (parsed_expr, _) = parse_risp_exprs(&tokenize(&expr))?;
    Ok(eval(&parsed_expr, env)?)
}

pub fn tokenize(expr: &str) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
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

        let (expr, new_xs) = parse_risp_exprs(&xs)?;
        res.push(expr);
        xs = new_xs;
    }
}

fn eval(expr: &RispExpr, env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    match expr {
        RispExpr::Bool(_) => Ok(expr.clone()),
        RispExpr::Symbol(symbol) => env
            .get(symbol)
            .ok_or(RispErr::Reason(format!("unexpected symbol, {}", symbol)))
            .map(|x| x.clone()),
        RispExpr::Number(_) => Ok(expr.clone()),
        RispExpr::List(list) => {
            let first_formula = list
                .first()
                .ok_or(RispErr::Reason("unexpected empty list".to_string()))?;
            let arg_formulas = &list[1..];

            match eval_built_in_form(first_formula, arg_formulas, env) {
                Some(res) => res,
                None => {
                    let first_eval = eval(first_formula, env)?;
                    // function 1 2 3のようなformat(functionは例えばdefault_env()で登録した和算関数)のListで最初が関数でない場合はinvalid formatであるというイメージ
                    // 仮にfunction 1 + 2だった場合はparse_float_argsでErr(RispErr)を返す
                    match first_eval {
                        RispExpr::Func(f) => {
                            let args_eval = arg_formulas
                                .iter()
                                .map(|x| eval(x, env))
                                .collect::<Result<Vec<RispExpr>, RispErr>>();
                            let risp_exprs = &args_eval?;
                            f(risp_exprs)
                        }
                        _ => Err(RispErr::Reason("first form must be a function".to_string())),
                    }
                }
            }
        }
        RispExpr::Func(_) => Err(RispErr::Reason("unexpected form".to_string())),
    }
}

fn eval_built_in_form(
    expr: &RispExpr,
    arg_forms: &[RispExpr],
    env: &mut RispEnv,
) -> Option<Result<RispExpr, RispErr>> {
    match expr {
        RispExpr::Symbol(s) => match s.as_str() {
            "if" => Some(eval_if_args(arg_forms, env)),
            "def" => Some(eval_def_args(arg_forms, env)),
            _ => None,
        },
        _ => None,
    }
}

// (if (= 1 1) 1 2)のような式を評価する(RispExprの形としてはList[List[Symbol("="), Number(1), Number(1)], Number(1), Number(2)]になる)
// ifの後の式がtest_formulaに入りtrueなら1(つまりarg_formula[1])が, falseなら2(つまりarg_formula[2])が評価される
fn eval_if_args(arg_formulas: &[RispExpr], env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    let test_formula = arg_formulas
        .first()
        .ok_or(RispErr::Reason("expect a test formula".to_string()))?;

    // if (= 1 1) 1 2の例の場合だと, (= 1 1)つまりList[Symbol("="), Number(1), Number(1)]の評価になる
    let test_eval = eval(test_formula, env)?;
    match test_eval {
        RispExpr::Bool(b) => {
            let index = if b { 1 } else { 2 };
            let res_form = arg_formulas.get(index).ok_or(RispErr::Reason(format!(
                "index out of bound. index: {}",
                index
            )))?;

            eval(res_form, env)
        }
        _ => Err(RispErr::Reason(format!(
            "unexpected test form='{}'",
            test_formula.to_string(),
        ))),
    }
}

fn eval_def_args(arg_formulas: &[RispExpr], env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    let first_formula = arg_formulas
        .first()
        .ok_or(RispErr::Reason("expected first formula".to_string()))?;

    let first_str = match first_formula {
        RispExpr::Symbol(s) => Ok(s.clone()),
        _ => Err(RispErr::Reason(
            "expected first formula to be a symbol".to_string(),
        )),
    }?;

    let second_formula = arg_formulas
        .get(1)
        .ok_or(RispErr::Reason("expected second formula".to_string()))?;

    if arg_formulas.len() > 2 {
        return Err(RispErr::Reason(
            "def can only have two formulas".to_string(),
        ));
    }

    let second_eval = eval(second_formula, env)?;
    env.data.insert(first_str, second_eval);

    Ok(first_formula.clone())
}
