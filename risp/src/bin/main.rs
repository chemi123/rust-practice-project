use std::io::{self, stdout, Write};

use risp::{
    core::engine::parse_risp_exp_string,
    entity::{risp_env::RispEnv, risp_err::RispErr},
};

fn slurp_exp_string() -> String {
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

    let env = RispEnv::new();
    loop {
        print!("risp > ");
        stdout().flush().unwrap();
        let exp_string = slurp_exp_string();
        match parse_risp_exp_string(exp_string, &env) {
            Ok(res) => println!("{}", res),
            Err(e) => match e {
                RispErr::Reason(msg) => println!("Error: {}", msg),
            },
        }
    }
}
