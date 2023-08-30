use std::{rc::Rc, cell::RefCell};

use anyhow::Result;
use rustyline::{Editor, history::FileHistory, error::ReadlineError};

use crate::{env::Env, eval, lisp_expr::LispExpr};

pub fn run() -> Result<()> {
    let mut rl = Editor::<(), FileHistory>::new()?;
    let _ = rl.load_history(".mal-history");

    let mut env = Rc::new(RefCell::new(Env::new()));
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(input) => {
                if !input.trim().is_empty() {
                    let _ = rl.add_history_entry(&input)?;
                    let _ = rl.save_history(".lisp-rs-history")?;

                    let result = eval::eval_str(input.as_str(), &mut env);
                    match result {
                        Ok(lisp_expr) => match lisp_expr {
                            LispExpr::Void => (),
                            _ => println!("{}", lisp_expr),
                        }
                        Err(e) => eprintln!("{}", e),
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted.");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("Bye!");
                break;
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            },
        }
    }

    Ok(())
}