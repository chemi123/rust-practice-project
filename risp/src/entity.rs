use core::fmt;
use std::collections::HashMap;

#[derive(Clone)]
pub struct RispEnv {
    data: HashMap<String, RispExp>,
}

impl RispEnv {
    // defaultのenvを返す
    pub fn new() -> Self {
        let mut data = HashMap::new();
        data.insert(
            "+".to_string(),
            RispExp::Func(|exps| -> Result<RispExp, RispErr> {
                let sum = parse_list_of_float(exps)?
                    .iter()
                    .fold(0.0, |sum, a| sum + *a);
                Ok(RispExp::Number(sum))
            }),
        );

        data.insert(
            "-".to_string(),
            RispExp::Func(|exps| -> Result<RispExp, RispErr> {
                let floats = parse_list_of_float(exps)?;
                let first = floats
                    .first()
                    .ok_or(RispErr::Reason("expected at least one number".to_string()))?;
                let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + *a);

                Ok(RispExp::Number(first - sum_of_rest))
            }),
        );

        RispEnv { data }
    }

    pub fn get(&self, key: &String) -> Option<&RispExp> {
        self.data.get(key)
    }
}

fn parse_list_of_float(exps: &[RispExp]) -> Result<Vec<f64>, RispErr> {
    // parse_single_floatでErrが帰ってきた場合はそのままErrを返す
    exps.iter().map(|x| parse_single_float(x)).collect()
}

fn parse_single_float(exp: &RispExp) -> Result<f64, RispErr> {
    match exp {
        RispExp::Number(num) => Ok(*num),
        _ => Err(RispErr::Reason("exptected a number".to_string())),
    }
}

#[derive(Debug)]
pub enum RispErr {
    Reason(String),
}

#[derive(Clone)]
pub enum RispExp {
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
    Func(fn(&[RispExp]) -> Result<RispExp, RispErr>),
}

impl fmt::Display for RispExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            RispExp::Symbol(s) => s.clone(),
            RispExp::Number(n) => n.to_string(),
            RispExp::List(list) => {
                // to_string()はfmt::Displayを実装していると使えるため、ここで再帰的にfmtが呼ばれる
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(" "))
            }
            RispExp::Func(_) => "Function {}".to_string(),
        };
        write!(f, "{}", str)
    }
}
