use risp::core::{
    engine::{parse, tokenize},
    engine2::{parse2, tokenize2},
};

fn main() {
    let tokens = tokenize("( + 1 (- 2 3)) 1 2 3");
    let (risp_exp, str_vec) = parse(&tokens).unwrap();
    println!("{}", risp_exp.to_string());
    println!("{:?}", str_vec);
    println!();

    let mut tokens = tokenize2("( + 1 (- 2 3)) 1 2 3");
    let (risp_exp, str_vec) = parse2(&mut tokens).unwrap();
    println!("{}", risp_exp.to_string());
    println!("{:?}", str_vec);
}
