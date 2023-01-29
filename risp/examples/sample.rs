use risp::core::{
    engine::{parse, tokenize},
    engine2::{parse2, tokenize2},
};

fn main() {
    let tokens = tokenize("( + 1 (- 2 3)) 1 2 3");
    let (risp_exp, str_vec) = parse(&tokens).unwrap();
    println!("{}", risp_exp);
    println!("{:?}", str_vec);
    println!();

    let tokens = tokenize2("( + 1 (- 2 3)) 1 2 3");
    let (risp_exp, str_vec) = parse2(tokens).unwrap();
    println!("{}", risp_exp);
    println!("{:?}", str_vec);
}
