use anyhow::Result;

use crate::lisp_expr::LispExpr;

pub fn parse_tokens() -> Result<LispExpr> {
    Ok(LispExpr::Void)
}