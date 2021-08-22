use crate::expression::Expression;
use crate::lisptype::LispType;

pub fn tokenize(c: &str) -> Vec<String> {
    let tokens: Vec<String> = c
        .replace("\"", " \" ")
        .split('\"')
        .map(|ms| ms.to_string())
        .collect();
    let mut staged_tokens: Vec<String> = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        if i % 2 != 0 {
            staged_tokens.push(token[1..token.len() - 1].to_string());
        } else {
            let s_tokens: Vec<String> = token
                .replace("(", " ( ")
                .replace(")", " ) ")
                .split_whitespace()
                .map(|t| t.to_string())
                .collect();
            for t in s_tokens {
                staged_tokens.push(t)
            }
        }
    }
    staged_tokens
}

struct Frame(Vec<String>, usize);

impl Frame {
    pub fn new() -> Self {
	// the num indicates the number of
	// LispTypes that have to be popped from
	// the res to be included in the parent LispType args.
	Self(vec![], 0)
    }

    pub fn push(&mut self, s: String) {
	self.0.push(s)
    }

    pub fn pop(&mut self) -> String {
	self.0.pop().unwrap()
    }
}

pub fn create_ast(tokens: Vec<String>) -> LispType {
    // Use it in reverse order. Doesen't matter in the end.
    let mut tokens: Vec<String> = tokens.iter().rev().map(|s| s.to_string()).collect();
    
    let mut stack: Vec<Frame> = vec![];
    let mut res: Vec<LispType> = vec![];
    loop {
	if tokens.len() == 0 {
	    break;
	}

	let mut new: Frame = Frame::new();
	let token = tokens.pop().unwrap();
	match token.as_ref() {
	    "(" => stack.push(Frame::new()),
	    ")" => new = stack.pop().unwrap(),
	    _ => stack.last_mut().unwrap().push(token.to_string()),
	}

	if new.0.len() != 0 {
	    let name  = new.pop();
	    let args = new.0.iter().map(|l| LispType::new(&[l.to_string()])).collect();
	    res.push(LispType::Expression(Expression::create(&name, args)))
	}
    }

    res.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenize() {
        let test = "(concat \"H W\" 2)";
        let exp: Vec<String> = vec!["(", "concat", "H W", "2", ")"]
            .iter()
            .map(|t| t.to_string())
            .collect();
        assert_eq!(tokenize(test), exp);
    }

    #[test]
    fn test_create_ast() {
	let test = "(* (+ 3 4) 2)";
	let tokens = tokenize(test);
	let mut res = create_ast(tokens);
	assert_eq!(res.run(&mut vec![]).num(&mut vec![]), 14.);
    }
}
