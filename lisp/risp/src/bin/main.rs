use std::io::{self, stdout, Write};

use risp::{
    core::parse_risp_expr_string,
    entity::{RispEnv, RispErr},
};

fn slurp_expr_string() -> String {
    let mut expr = String::new();
    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    expr
}

fn main() {
    /* 究極的には以下の感じのコードの方が望ましい. lifetimeの問題に当たって難しかったので, 後でengine2の方針で実装してみる
        let risp = Risp::new();
        risp.run();
    */

    let mut env = RispEnv::new();
    loop {
        print!("risp > ");
        stdout().flush().unwrap();
        let expr_string = slurp_expr_string();
        match parse_risp_expr_string(expr_string, &mut env) {
            Ok(res) => println!("{}", res),
            Err(e) => match e {
                RispErr::Reason(msg) => println!("Error: {}", msg),
            },
        }
    }
}
