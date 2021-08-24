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

struct Frame(Vec<String>, usize, bool);

impl Frame {
    pub fn new() -> Self {
        // the num indicates the number of
        // LispTypes that have to be popped from
        // the res to be included in the parent LispType args.
        Self(vec![], 0, false)
    }

    pub fn push(&mut self, s: String) {
        self.0.push(s)
    }

    pub fn pop(&mut self) -> String {
        self.0.pop().unwrap()
    }

    pub fn set_done(&mut self) {
        self.2 = true;
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
        let mut new_args: Vec<Frame> = vec![];
        match token.as_ref() {
            "(" => stack.push(Frame::new()),
            ")" => {
                new_args = get_args_from_stack(&mut stack);
                new = stack.pop().unwrap();
            }
            _ => stack.last_mut().unwrap().push(token.to_string()),
        }

        if new.0.len() != 0 {
            let name = new.pop();
            let args = new_args
                .iter()
                .map(|e| {
                    LispType::new(
                        &e.0.iter()
                            .rev()
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>(),
                    )
                })
                .collect();
            res.push(LispType::Expression(Expression::create(&name, args)))
        }
    }

    res.pop().unwrap()
}

fn get_args_from_stack(stack: &mut Vec<Frame>) -> Vec<Frame> {
    let mut res: Vec<Frame> = vec![];
    loop {
        if !stack.last().unwrap().2 {
            res.push(stack.pop().unwrap());
        } else {
            break;
        }
    }
    res
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

    #[test]
    fn test_get_args_from_stack() {
        let mut test = vec![Frame::new(), Frame::new(), Frame::new(), Frame::new()];
        test[1].set_done();
        assert_eq!(get_args_from_stack(&mut test).len(), 2);
        assert_eq!(test.len(), 2)
    }
}
