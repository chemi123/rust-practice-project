use std::{rc::Rc, cell::RefCell};

use anyhow::{Result, bail};

use crate::{lisp_expr::LispExpr, env::Env, lexer::tokenize, parser::parse};

pub fn eval_str(expr_str: &str, env: &mut Rc<RefCell<Env>>) -> Result<LispExpr> {
    let tokens = tokenize(expr_str)?;
    let lisp_expr = parse(tokens)?;
    eval_lispexpr(&lisp_expr, env)
}

fn eval_lispexpr(lisp_expr: &LispExpr, env: &mut Rc<RefCell<Env>>) -> Result<LispExpr> {
    match lisp_expr {
        LispExpr::Void => Ok(LispExpr::Void),
        LispExpr::Lambda(_, _) => Ok(LispExpr::Void),
        LispExpr::Integer(n) => Ok(LispExpr::Integer(*n)),
        LispExpr::Bool(b) => Ok(LispExpr::Bool(*b)),
        LispExpr::Symbol(s) => eval_symbol(s, env),
        LispExpr::List(list) => eval_list(list, env),
    }
}

fn eval_symbol(symbol: &str, env: &mut Rc<RefCell<Env>>) -> Result<LispExpr> {
    let val = env.borrow().get(symbol);
    if val.is_none() {
        bail!("Unbound symbol: {}", symbol)
    }
    Ok(val.unwrap())
}

fn eval_list(lisp_exprs: &Vec<LispExpr>, env: &mut Rc<RefCell<Env>>) -> Result<LispExpr> {
    let head = &lisp_exprs[0];
    match head {
        LispExpr::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => eval_binary_operator(lisp_exprs, env),
            "define" => eval_define(lisp_exprs, env),
            "if" => eval_if(lisp_exprs, env),
            "lambda" => eval_function_definition(lisp_exprs),
            _ => eval_function_call(s, lisp_exprs, env),
        },
        _ => {
            let mut result = Vec::new();
            for lisp_expr in lisp_exprs {
                let evaluated_lisp_expr = eval_lispexpr(lisp_expr, env)?;
                match evaluated_lisp_expr {
                    LispExpr::Void => (),
                    _ => result.push(evaluated_lisp_expr),
                }
            }
            Ok(LispExpr::List(result))
        },
    }
}

fn eval_binary_operator(lisp_exprs: &Vec<LispExpr>, env: &mut Rc<RefCell<Env>>) -> Result<LispExpr> {
    if lisp_exprs.len() != 3 {
        bail!("Invalid number of arguments for binary operator")
    }

    let left = eval_lispexpr(&lisp_exprs[1], env)?;
    let left = match left {
        LispExpr::Integer(n) => n,
        _ => bail!("Left operand must be a number")
    };

    let right = eval_lispexpr(&lisp_exprs[2], env)?;
    let right = match right {
        LispExpr::Integer(n) => n,
        _ => bail!("Left operand must be a number")
    };

    let result = match &lisp_exprs[0] {
        LispExpr::Symbol(operator) => match operator.as_str() {
            "+" => LispExpr::Integer(left + right),
            "-" => LispExpr::Integer(left - right),
            "*" => LispExpr::Integer(left * right),
            "/" => LispExpr::Integer(left / right),
            ">" => LispExpr::Bool(left > right),
            "<" => LispExpr::Bool(left < right),
            "=" => LispExpr::Bool(left == right),
            "!=" => LispExpr::Bool(left != right),
            _ => bail!("Invalid operator"),
        }
        _ => bail!("Invalid operator"),
    };

    Ok(result)
}

fn eval_define(lisp_exprs: &Vec<LispExpr>, env: &mut Rc<RefCell<Env>>) -> Result<LispExpr> {
    if lisp_exprs.len() != 3 {
        bail!("Invalid number of arguments for define")
    }

    let val = eval_lispexpr(&lisp_exprs[2], env)?;
    if let LispExpr::Void = val {
        bail!("Unable to assign Void to a variable")
    }

    match &lisp_exprs[1] {
        LispExpr::Symbol(s) => env.borrow_mut().set(s, val),
        _ => bail!("Invalid define")
    }

    Ok(LispExpr::Void)
}

fn eval_if(lisp_exprs: &Vec<LispExpr>, env: &mut Rc<RefCell<Env>>) -> Result<LispExpr> {
    if lisp_exprs.len() != 4 {
        bail!("Invalid number of arguments for if")
    }

    let condition = eval_lispexpr(&lisp_exprs[1], env)?;
    let condition = match condition {
        LispExpr::Bool(b) => b,
        _ => bail!("Operand after if must be a Bool"),
    };

    if condition {
        eval_lispexpr(&lisp_exprs[2], env)
    } else {
        eval_lispexpr(&lisp_exprs[3], env)
    }
}

fn eval_function_definition(lisp_exprs: &Vec<LispExpr>) -> Result<LispExpr> {
    if lisp_exprs.len() != 3 {
        bail!("Invalid number of arguments for lambda")
    }

    let params = match &lisp_exprs[1] {
        LispExpr::List(list) => {
            let mut params = Vec::new();
            for lisp_expr in list {
                match lisp_expr {
                    LispExpr::Symbol(s) => params.push(s.clone()),
                    _ => bail!("Parameter must be a symbol"),
                }
            }
            params
        },
        _ => bail!("Parameters must be a list"),
    };

    match &lisp_exprs[2] {
        LispExpr::List(list) => Ok(LispExpr::Lambda(params, list.clone())),
        _ => bail!("Body must be a list"),
    }
}

fn eval_function_call(symbol: &str, lisp_exprs: &Vec<LispExpr>, env: &mut Rc<RefCell<Env>>) -> Result<LispExpr> {
    let lambda = env.borrow().get(symbol);
    if lambda.is_none() {
        bail!("Unbound symbol: {}", symbol)
    }

    let function = lambda.unwrap();
    match function {
        LispExpr::Lambda(params, body) => {
            let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
            for (i, param) in params.iter().enumerate() {
                let lisp_expr = eval_lispexpr(&lisp_exprs[i+1], env)?;
                new_env.borrow_mut().set(param, lisp_expr);
            }
            eval_lispexpr(&LispExpr::List(body), &mut new_env)
        },
        _ => bail!("Not a lambda"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}