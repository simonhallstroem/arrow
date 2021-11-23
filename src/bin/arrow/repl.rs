use arrow::tokenize::{ast, create_lisptypes};
use arrow::Arrow;

const MESSAGE: &str = "Arrow: A LISP dialect.
Version: ";

const REPL_HELP: &str = "REPL COMMANDS:
    exit     exit this repl session.
    help     print this message.
    version  print the Arrow version.";

pub fn repl() {
    use std::io::Write;
    use std::io::{stdin, stdout};

    println!("{}{}", MESSAGE, env!("CARGO_PKG_VERSION"));

    let mut lispfns: Arrow = Arrow::default();

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
            "version" => println!("{}{}", MESSAGE, env!("CARGO_PKG_VERSION")),
            _ => {
                if input.len() > 7 {
                    match input.trim_start()[0..6].as_ref() {
                        "(defun" => {
                            lispfns = lispfns.add_function(&input).unwrap();
                        }
                        _ => println!(
                            "{}",
                            lispfns.run(&input).unwrap().to_string(&mut vec![]).unwrap()
                        ),
                    }
                } else if input.chars().nth(0).unwrap() == '(' {
                    let ast = ast(input);
                    let lisptype = &mut create_lisptypes(ast).unwrap()[0];
                    println!(
                        "{}",
                        lisptype
                            .run(&mut vec![])
                            .unwrap()
                            .to_string(&mut vec![])
                            .unwrap()
                    );
                } else {
                    println!(
                        "{}",
                        lispfns.run(&input).unwrap().to_string(&mut vec![]).unwrap()
                    )
                }
            }
        }
    }
}
