use core::fmt;

use super::risp_err::RispErr;

#[derive(Clone)]
pub enum RispExp {
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
    Func(fn(&[RispExp]) -> Result<RispExp, RispErr>),
}

pub fn parse_list_of_float(exps: &[RispExp]) -> Result<Vec<f64>, RispErr> {
    // parse_single_floatでErrが帰ってきた場合はそのままErrを返す
    exps.iter().map(|x| parse_single_float(x)).collect()
}

pub fn parse_single_float(exp: &RispExp) -> Result<f64, RispErr> {
    match exp {
        RispExp::Number(num) => Ok(*num),
        _ => Err(RispErr::Reason("exptected a number".to_string())),
    }
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
