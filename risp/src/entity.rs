use core::fmt;
use std::collections::HashMap;

use crate::{ensure_tonicity, utils::parse_list_of_floats};

#[derive(Clone)]
pub struct RispEnv {
    data: HashMap<String, RispExpr>,
}

impl RispEnv {
    // defaultのenvを返す
    pub fn new() -> Self {
        let mut data = HashMap::new();
        data.insert(
            "+".to_string(),
            RispExpr::Func(|exprs| -> Result<RispExpr, RispErr> {
                let sum = parse_list_of_floats(exprs)?
                    .iter()
                    .fold(0.0, |sum, a| sum + *a);
                Ok(RispExpr::Number(sum))
            }),
        );

        data.insert(
            "-".to_string(),
            RispExpr::Func(|exprs| -> Result<RispExpr, RispErr> {
                let floats = parse_list_of_floats(exprs)?;
                let first = floats
                    .first()
                    .ok_or(RispErr::Reason("expected at least one number".to_string()))?;
                let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + *a);

                Ok(RispExpr::Number(first - sum_of_rest))
            }),
        );

        data.insert(
            "=".to_string(),
            RispExpr::Func(ensure_tonicity!(|a, b| a == b)),
        );

        data.insert(
            ">".to_string(),
            RispExpr::Func(ensure_tonicity!(|a, b| a > b)),
        );

        data.insert(
            ">=".to_string(),
            RispExpr::Func(ensure_tonicity!(|a, b| a >= b)),
        );

        data.insert(
            "<".to_string(),
            RispExpr::Func(ensure_tonicity!(|a, b| a < b)),
        );

        data.insert(
            "<=".to_string(),
            RispExpr::Func(ensure_tonicity!(|a, b| a <= b)),
        );

        RispEnv { data }
    }

    pub fn get(&self, key: &String) -> Option<&RispExpr> {
        self.data.get(key)
    }
}

#[derive(Debug)]
pub enum RispErr {
    Reason(String),
}

#[derive(Clone)]
pub enum RispExpr {
    Bool(bool),
    Symbol(String),
    Number(f64),
    List(Vec<RispExpr>),
    Func(fn(&[RispExpr]) -> Result<RispExpr, RispErr>),
}

impl fmt::Display for RispExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            RispExpr::Bool(a) => a.to_string(),
            RispExpr::Symbol(s) => s.clone(),
            RispExpr::Number(n) => n.to_string(),
            RispExpr::List(list) => {
                // to_string()はfmt::Displayを実装していると使えるため、ここで再帰的にfmtが呼ばれる
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(" "))
            }
            RispExpr::Func(_) => "Function {}".to_string(),
        };
        write!(f, "{}", str)
    }
}
