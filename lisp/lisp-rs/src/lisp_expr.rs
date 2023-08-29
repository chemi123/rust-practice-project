use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum LispExpr {
    Void,
    Integer(i64),
    Symbol(String),
    Lambda(Vec<String>, Vec<LispExpr>),
    List(Vec<LispExpr>),
}

impl Display for LispExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LispExpr::Void => Ok(()),
            LispExpr::Integer(n) => write!(f, "{}", n),
            LispExpr::Symbol(s) => write!(f, "{}", s),
            LispExpr::Lambda(params, lisp_exprs) => {
                let lisp_exprs_string = lisp_exprs.iter()
                    .map(|lisp_expr| lisp_expr.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "(lambda ({}) ({}))", params.join(" "), lisp_exprs_string)
            },
            LispExpr::List(lisp_exprs) =>  {
                let lisp_exprs_string = lisp_exprs.iter()
                    .map(|lisp_expr| lisp_expr.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "({})", lisp_exprs_string)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_display() {
        let lisp_exprs = LispExpr::Lambda(vec!["x".to_string(), "y".to_string()], vec![
            LispExpr::Symbol("+".to_string()),
            LispExpr::Integer(1),
            LispExpr::Integer(1),
            LispExpr::Void,
        ]);
        println!("{}", lisp_exprs);
    }
}