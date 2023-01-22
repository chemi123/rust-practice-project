use risp::{
    core,
    entity::{
        risp_env::RispEnv,
        risp_err::RispErr,
        risp_exp::{print_risp_exp, RispExp},
    },
};
fn main() {
    let exp = RispExp::Symbol(String::from("str"));
    let err = RispErr::Reason(String::from("reason"));
    let env = RispEnv::new();
    println!("{:?} {:?} {:?}", print_risp_exp(&exp), err, env);
    println!("{:?}", core::engine::tokenize(&String::from("(+ 10 1)")));
}
