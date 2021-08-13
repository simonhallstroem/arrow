mod repl;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        repl::repl();
    } else {
        let _code = handle_args(args);
    }
}

fn handle_args(a: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    a.iter().for_each(|a| match a.as_str() {
        "--version" | "-v" => version(),
        "--help" | "-h" => help(),
        _ => res.push(a.to_string()),
    });
    res
}

fn version() {
    println!("Arrow version: {}", env!("CARGO_PKG_VERSION"));
}

fn help() {
    println!(
        "USAGE:
    arrow [ARGS] | [CODE]

ARGS: 
    --version | -v   Print the version of arrow.
    --help | -h      Print this message.

CODE:
This can be filled with standard LISP code. 
Arrow will then try to evaluate it."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_args() {
        let test = vec!["--version", "-h", "(message", "\"h\")"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let exp: Vec<String> = vec!["(message", "\"h\")"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(handle_args(test), exp);
    }
}
