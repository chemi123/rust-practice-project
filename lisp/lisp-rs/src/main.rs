use std::{rc::Rc, cell::RefCell, error::Error};

use env::Env;
use linefeed::{Interface, ReadResult};

use crate::parser::Object;

mod env;
mod eval;
mod lexer;
mod parser;

const PROMPT: &str = "lisp-rs> ";

fn main() -> Result<(), Box<dyn Error>> {
    let reader = Interface::new(PROMPT).unwrap();
    let mut env = Rc::new(RefCell::new(Env::new()));

    reader.set_prompt(format!("{}", PROMPT).as_ref())?;
    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }

        let mut tokens = lexer::tokenize(input.as_str())?;
        let object = parser::parse_tokens(&mut tokens)?;
        let object = eval::eval_obj(&object, &mut env)?;

        match object {
            Object::Void => (),
            Object::Integer(n) => println!("{}", n),
            Object::Bool(b) => println!("{}", b),
            Object::Symbol(s) => println!("{}", s),
            Object::Lambda(params, body) => {
                println!("Lambda(");
                for param in params {
                    println!("{} ", param);
                }
                println!(")");
                for expr in body {
                    println!(" {}", expr);
                }
            },
            _ => println!("{}", object),
        }
    }

    println!("good bye!");
    Ok(())
}
