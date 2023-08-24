use std::{cell::RefCell, rc::Rc};

use anyhow::{Result, anyhow};

use crate::{env::Env, parser::Object};

pub fn eval_obj(object: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object> {
    match object {
        Object::Void => Ok(Object::Void),
        Object::Lambda(_, _) => Ok(Object::Void),
        Object::Bool(b) => Ok(Object::Bool(*b)),
        Object::Integer(i) => Ok(Object::Integer(*i)),
        Object::Symbol(s) => eval_symbol(s, env),
        Object::List(list) => eval_list(list, env),
    }
}

fn eval_symbol(s: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object> {
    let val = env.borrow().get(s);
    if val.is_none() {
        return Err(anyhow!("Unbound symbol: {}", s))
    }
    Ok(val.unwrap().clone())
}

fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object> {
    let head = &list[0];
    match head {
        Object::Symbol(s) => match s.as_ref() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => eval_binary_operator(list, env),
            "if" => eval_if(list, env),
            "define" => eval_define(list, env),
            "lambda" => eval_function_definition(list),
            _ => eval_function_call(s.as_str(), list, env),
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = eval_obj(obj, env)?;
                match result {
                    Object::Void => (),
                    _ => new_list.push(result),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

fn eval_binary_operator(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object> {
    if list.len() != 3 {
        return Err(anyhow!("Invalid number of arguments for binary operator"));
    }

    let left = eval_obj(&list[1], env)?;
    let left = match left {
        Object::Integer(n) => n,
        _ => return Err(anyhow!("Left operand must be an integer")),
    };

    let right = eval_obj(&list[2], env)?;
    let right = match right {
        Object::Integer(n) => n,
        _ => return Err(anyhow!("Right operand must be an integer")),
    };

    let operator = &list[0];
    match operator {
        Object::Symbol(s) => match s.as_str() {
            "+" => Ok(Object::Integer(left + right)),
            "-" => Ok(Object::Integer(left - right)),
            "*" => Ok(Object::Integer(left * right)),
            "/" => Ok(Object::Integer(left / right)),
            "<" => Ok(Object::Bool(left < right)),
            ">" => Ok(Object::Bool(left > right)),
            "=" => Ok(Object::Bool(left == right)),
            "!=" => Ok(Object::Bool(left != right)),
            _ => Err(anyhow!("Invalid operator: {}", s)),
        },
        _ => Err(anyhow!("Operator must be a symbol")),
    }
}

fn eval_if(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object> {
    if list.len() != 4 {
        return Err(anyhow!("Invalid number of arguments for if statement"));
    }

    let condition = eval_obj(&list[1], env)?;
    let condition = match condition {
        Object::Bool(b) => b,
        _ => return Err(anyhow!("Condition must be a boolean")),
    };

    if condition {
        eval_obj(&list[2], env)
    } else {
        eval_obj(&list[3], env)
    }
}

fn eval_define(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object> {
    if list.len() != 3 {
        return Err(anyhow!("Invalid number of arguments for define"));
    }

    let var = match &list[1] {
        Object::Symbol(s) => s.clone(),
        _ => return Err(anyhow!("Invalid define")),
    };

    let obj = eval_obj(&list[2], env)?;
    env.borrow_mut().set(var.as_str(), obj);
    Ok(Object::Void)
}

fn eval_function_definition(list: &Vec<Object>) -> Result<Object> {
    if list.len() != 3 {
        return Err(anyhow!("Invalid number of arguments for lambda"));
    }

    let params = match &list[1] {
        Object::List(list) => {
            let mut args = Vec::new();
            for obj in list {
                match obj {
                    Object::Symbol(s) => args.push(s.clone()),
                    _ => return Err(anyhow!("Invalid lambda argument")),
                }
            }
            args
        }
        _ => return Err(anyhow!("Invalid lambda")),
    };

    let body = match &list[2] {
        Object::List(list) => list.clone(),
        _ => return Err(anyhow!("Invalid lambda")),
    };

    Ok(Object::Lambda(params, body))
}

fn eval_function_call(
    key: &str,
    list: &Vec<Object>,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object> {
    let lambda = env.borrow().get(key);
    if lambda.is_none() {
        return Err(anyhow!("Unbound symbol: {}", key));
    }

    let func = lambda.unwrap();
    match func {
        Object::Lambda(params, body) => {
            let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
            for (i, param) in params.iter().enumerate() {
                let val = eval_obj(&list[i + 1], env)?;
                new_env.borrow_mut().set(param, val);
            }
            eval_obj(&Object::List(body), &mut new_env)
        }
        _ => Err(anyhow!("Not a lambda: {}", key)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {}
}
