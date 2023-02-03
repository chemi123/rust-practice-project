use std::num::ParseFloatError;

use crate::entity::{RispErr, RispExpr};

pub fn parse_atom(token: &str) -> RispExpr {
    match token {
        "true" => RispExpr::Bool(true),
        "false" => RispExpr::Bool(false),
        _ => {
            let potential_float: Result<f64, ParseFloatError> = token.parse();
            match potential_float {
                Ok(v) => RispExpr::Number(v),
                Err(_) => RispExpr::Symbol(token.to_string()),
            }
        }
    }
}

pub fn parse_list_of_floats(exprs: &[RispExpr]) -> Result<Vec<f64>, RispErr> {
    // parse_single_floatでErrが帰ってきた場合はそのままErrを返す
    exprs.iter().map(|x| parse_single_float(x)).collect()
}

pub fn parse_single_float(expr: &RispExpr) -> Result<f64, RispErr> {
    match expr {
        RispExpr::Number(num) => Ok(*num),
        _ => Err(RispErr::Reason("expected a number".to_string())),
    }
}
