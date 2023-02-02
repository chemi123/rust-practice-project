use risp::{
    core::{parse_risp_expressions, tokenize},
    core_2::{parse2, tokenize2},
};

fn main() {
    let tokens = tokenize("( + 1 (- 2 3)) 1 2 3");
    let (risp_expr, str_vec) = parse_risp_expressions(&tokens).unwrap();
    println!("{}", risp_expr);
    println!("{:?}", str_vec);
    println!();

    let tokens = tokenize2("( + 1 (- 2 3)) 1 2 3");
    let (risp_expr, str_vec) = parse2(tokens).unwrap();
    println!("{}", risp_expr);
    println!("{:?}", str_vec);
}
