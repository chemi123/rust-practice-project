use core::fmt;
use std::collections::HashMap;

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
                let sum = parse_list_of_float(exprs)?
                    .iter()
                    .fold(0.0, |sum, a| sum + *a);
                Ok(RispExpr::Number(sum))
            }),
        );

        data.insert(
            "-".to_string(),
            RispExpr::Func(|exprs| -> Result<RispExpr, RispErr> {
                let floats = parse_list_of_float(exprs)?;
                let first = floats
                    .first()
                    .ok_or(RispErr::Reason("expected at least one number".to_string()))?;
                let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + *a);

                Ok(RispExpr::Number(first - sum_of_rest))
            }),
        );

        RispEnv { data }
    }

    pub fn get(&self, key: &String) -> Option<&RispExpr> {
        self.data.get(key)
    }
}

fn parse_list_of_float(exprs: &[RispExpr]) -> Result<Vec<f64>, RispErr> {
    // parse_single_floatでErrが帰ってきた場合はそのままErrを返す
    exprs.iter().map(|x| parse_single_float(x)).collect()
}

fn parse_single_float(expr: &RispExpr) -> Result<f64, RispErr> {
    match expr {
        RispExpr::Number(num) => Ok(*num),
        _ => Err(RispErr::Reason("expected a number".to_string())),
    }
}

#[derive(Debug)]
pub enum RispErr {
    Reason(String),
}

#[derive(Clone)]
pub enum RispExpr {
    Symbol(String),
    Number(f64),
    List(Vec<RispExpr>),
    Func(fn(&[RispExpr]) -> Result<RispExpr, RispErr>),
}

impl fmt::Display for RispExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
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
