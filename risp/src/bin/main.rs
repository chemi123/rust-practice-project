use risp::{
    core,
    entity::{risp_err::RispErr, risp_exp::RispExp},
};
fn main() {
    let exp = RispExp::Symbol(String::from("str"));
    let err = RispErr::Reason(String::from("reason"));
    println!("{} {:?}", exp, err,);
    println!("{:?}", core::engine::tokenize(&String::from("(+ 10 1)")));
}
