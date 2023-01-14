use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

fn main() {
    loop {
        let current_dir = env::current_dir().unwrap();
        print!("{} $ ", current_dir.display());
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    println!("{}", e);
                }
            }
            "exit" => return,
            _ => {
                let child = Command::new(command).args(args).spawn();

                match child {
                    Ok(mut c) => {
                        c.wait().unwrap();
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}
