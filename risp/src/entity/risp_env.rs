use std::collections::HashMap;

use super::{risp_err::RispErr, risp_exp::RispExp};

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
