use risp::{
    core::{parse, tokenize},
    core_2::{parse2, tokenize2},
};

fn main() {
    let tokens = tokenize("( + 1 (- 2 3)) 1 2 3");
    let (risp_expr, str_vec) = parse(&tokens).unwrap();
    println!("{}", risp_expr);
    println!("{:?}", str_vec);
    println!();

    let tokens = tokenize2("( + 1 (- 2 3)) 1 2 3");
    let (risp_expr, str_vec) = parse2(tokens).unwrap();
    println!("{}", risp_expr);
    println!("{:?}", str_vec);
}
