mod lexer;
mod lisp_expr;
mod parser;
mod token;

use anyhow::Result;
use rustyline::{Editor, history::FileHistory, error::ReadlineError};

fn main() -> Result<()> {
    let mut rl = Editor::<(), FileHistory>::new()?;
    let _ = rl.load_history(".mal-history");

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(input) => {
                if !input.trim().is_empty() {
                    println!("{}", input);
                    let _ = rl.add_history_entry(&input)?;
                    let _ = rl.save_history(".lisp-rs-history")?;
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