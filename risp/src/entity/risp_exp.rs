use super::risp_err::RispErr;

#[derive(Clone)]
pub enum RispExp {
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
    Func(fn(&[RispExp]) -> Result<RispExp, RispErr>),
}

// 要素にFuncがあるため、RispExpはDebugをderiveできない.
// デバッグしたかったので実装. 再帰処理になっており, RispExp自体もListで再帰構造になっているため, RispExpを再帰しすぎるとスタックオーバーフローするのと, しなくても見づらくなる
pub fn print_risp_exp(exp: &RispExp) {
    match exp {
        RispExp::Symbol(s) => print!("Symbol({}) ", s),
        RispExp::Number(f) => print!("Number({}) ", f),
        RispExp::List(v) => {
            print!("{{ ");
            for e in v.iter() {
                print_risp_exp(e)
            }
            println!("}}");
        }
        RispExp::Func(_) => print!("Function "),
    }
}
