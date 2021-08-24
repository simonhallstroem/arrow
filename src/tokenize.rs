use crate::expression::Expression;
use crate::lisptype::LispType;

pub fn tokenize(c: &str) -> Vec<AstType> {
    let tokens: Vec<String> = c
        .replace("\"", " \" ")
        .split('\"')
        .map(|ms| ms.to_string())
        .collect();
    let mut staged_tokens: Vec<AstType> = Vec::new();
    let mut last_openbracket: usize = 0;
    for (i, token) in tokens.iter().enumerate() {
        if i % 2 != 0 {
            staged_tokens.push(AstType::Type(LispType::String(
                token[1..token.len() - 1].to_string(),
            )));
        } else {
            let s_tokens: Vec<String> = token
                .replace("(", " ( ")
                .replace(")", " ) ")
                .split_whitespace()
                .map(|t| t.to_string())
                .collect();
            for t in s_tokens {
                staged_tokens.push(match t.as_str() {
                    "(" => {
                        last_openbracket = 0;
                        AstType::OpenBracket
                    }
                    ")" => {
                        last_openbracket += 1;
                        AstType::ClosedBracket
                    }
                    _ => {
                        last_openbracket += 1;
                        if last_openbracket == 1 {
                            AstType::FnName(t)
                        } else {
                            AstType::Type(LispType::new(&[t]))
                        }
                    }
                })
            }
        }
    }
    staged_tokens
}

pub enum AstType {
    Type(LispType),
    FnName(String),
    OpenBracket,
    ClosedBracket,
}

impl ToString for AstType {
    fn to_string(&self) -> String {
        match self {
            Self::Type(t) => t.to_string(&mut vec![]),
            Self::FnName(s) => s.to_string(),
            Self::OpenBracket => "(".to_string(),
            Self::ClosedBracket => ")".to_string(),
        }
    }
}

pub fn create_ast(tokens: Vec<AstType>) -> LispType {
    let mut stack: Vec<AstType> = vec![];

    for token in tokens {
        if let AstType::ClosedBracket = token {
            let mut temp_stack: Vec<AstType> = vec![];
            loop {
                let ctoken = stack.pop().unwrap();

                match ctoken {
                    AstType::OpenBracket => {
                        stack.push(AstType::Type(LispType::Expression(Expression::create(
                            &temp_stack.pop().unwrap().to_string(),
                            temp_stack
                                .iter()
                                .map(|e| match e {
                                    AstType::Type(t) => t.clone(),
                                    _ => panic!(""),
                                })
                                .collect::<Vec<LispType>>(),
                        ))))
                    }
                    _ => temp_stack.push(stack.pop().unwrap()),
                }
            }
        } else {
            stack.push(token)
        }
    }

    LispType::Bool(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenize() {
        use AstType::*;
        let test = "(concat \"H W\" 2)";
        let exp: Vec<AstType> = vec![
            OpenBracket,
            FnName("concat".to_string()),
            Type(LispType::String("H W".to_string())),
            Type(LispType::Number(2.)),
            ClosedBracket,
        ];
        let res = tokenize(test);
        let _ = res
            .iter()
            .zip(exp)
            .map(|(e, t)| assert_eq!(e.to_string(), t.to_string()));
    }

    #[test]
    fn test_create_ast() {
        let test = "(* (+ 3 4) 2)";
        let tokens = tokenize(test);
        let mut res = create_ast(tokens);
        assert_eq!(res.run(&mut vec![]).num(&mut vec![]), 14.);
    }

    // #[test]
    // fn test_get_args_from_stack() {
    //     let mut test = vec![Frame::new(), Frame::new(), Frame::new(), Frame::new()];
    //     test[1].set_done();
    //     assert_eq!(get_args_from_stack(&mut test).len(), 2);
    //     assert_eq!(test.len(), 2)
    // }
}
