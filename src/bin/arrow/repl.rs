const MESSAGE: &str = "Arrow: A LISP dialect.
Version: ";

const REPL_HELP: &str = "REPL COOMMANDS:
    exit     exit this repl session.
    help     print this message.";

pub fn repl() {
    use std::io::Write;
    use std::io::{stdin, stdout};

    println!("{}{}", MESSAGE, env!("CARGO_PKG_VERSION"));

    loop {
        let mut input = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap_or_else(|_| {
            println!("Invalid string.");
            0
        });

        let input = input.trim();

        match input {
            "exit" => break,
            "help" => println!("{}", REPL_HELP),
            _ => println!("Command not recognized."),
        }
    }
}
