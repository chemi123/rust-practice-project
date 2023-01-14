use colored::*;
use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::{Child, Command, Stdio},
};

fn main() {
    loop {
        let current_dir = env::current_dir().unwrap();
        print!(
            "{} {}",
            current_dir.display().to_string().cyan(),
            "$ ".white()
        );
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // peekableは次の値を"consume"しないで覗き見することができるiterator. 名前のまんま
        let mut commands = input.trim().split("|").peekable();
        let mut previous_command = None;

        // "|"で区切られたコマンドをiterateして処理する
        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        println!("{}", e);
                    }
                    previous_command = None;
                }
                "exit" => return,
                // 例えばコマンドが`cat file.txt | grep something`の時を例にして考えてみる
                _ => {
                    // - previous_commandがNone, つまりcommandがcatの場合
                    //     Stdio::inherit()で親プロセスの標準入力を受け取る -> terminalに表示されている画面のカーソルから打ち込まれた文字列が標準入力になる
                    // - previout_commandがSome, つまりcommandがgrepの場合
                    //     前回のコマンド`cat file.txt`が子プロセスで実行されており、その標準出力を標準入力にする.
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        // まだ後続にパイプ処理が残っている場合, 標準出力をpipeとして繋げる
                        Stdio::piped()
                    } else {
                        // 最後のコマンドの場合, 標準出力を親プロセス(terminalの出力)の方に切り替える. でないと結果が画面に出力されない
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => previous_command = Some(output),
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    }
                }
            }
        }

        if let Some(mut last_command) = previous_command {
            // pipe最後のコマンドの処理実行が完了するまで待つ
            last_command.wait().unwrap();
        }
    }
}
