use crate::expression::Expression;
use crate::lisptype::LispType;

pub fn tokenize(c: &str) -> Result<Vec<AstType>, &'static str> {
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
                            AstType::Type(LispType::new(&[t])?)
                        }
                    }
                })
            }
        }
    }
    Ok(staged_tokens)
}

#[derive(Debug)]
pub enum AstType {
    Type(LispType),
    FnName(String),
    OpenBracket,
    ClosedBracket,
}

impl ToString for AstType {
    fn to_string(&self) -> String {
        match self {
            Self::Type(t) => t.to_string(&mut vec![]).unwrap(),
            Self::FnName(s) => s.to_string(),
            Self::OpenBracket => "(".to_string(),
            Self::ClosedBracket => ")".to_string(),
        }
    }
}

pub fn create_ast(tokens: Vec<AstType>) -> Result<LispType, &'static str> {
    let mut stack: Vec<AstType> = vec![];

    for token in tokens {
        if let AstType::ClosedBracket = token {
            let mut temp_stack: Vec<AstType> = vec![];
            loop {
                let ctoken = if let Some(s) = stack.pop() {
                    s
                } else {
                    break;
                };

                let res: Vec<LispType> = vec![];

                match ctoken {
                    AstType::ClosedBracket => {
                        stack.push(AstType::Type(LispType::Expression(Expression::create(
                            &match temp_stack.pop().unwrap() {
                                AstType::FnName(f) => f.to_string(),
                                _ => return Err("Invalid Syntax."),
                            },
                            {
                                loop {
                                    match temp_stack.pop().unwrap() {
                                        AstType::OpenBracket => break,
                                        AstType::Type(t) => t.clone(),
                                        _ => return Err("Invalid Syntax."),
                                    };
                                }
                                res
                            },
                        )?)))
                    }
                    _ => temp_stack.push(ctoken),
                }

                println!("{:?}", temp_stack);
            }
        } else {
            stack.push(token)
        }
        println!("Stack: {:?}", stack);
    }

    match stack.pop() {
        Some(s) => match s {
            AstType::Type(t) => Ok(t),
            _ => Err("Found non valid data on stack."),
        },
        None => Err("No data found on stack."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenize() -> Result<(), &'static str> {
        use AstType::*;
        let test = "(concat \"H W\" 2)";
        let exp: Vec<AstType> = vec![
            OpenBracket,
            FnName("concat".to_string()),
            Type(LispType::String("H W".to_string())),
            Type(LispType::Number(2.)),
            ClosedBracket,
        ];
        let res = tokenize(test)?;
        let _ = res
            .iter()
            .zip(exp)
            .map(|(e, t)| assert_eq!(e.to_string(), t.to_string()));
        Ok(())
    }

    #[test]
    fn test_create_ast() -> Result<(), &'static str> {
        let test = "(* (+ 3 4) 2)";
        let tokens = tokenize(test)?;
        let mut res = create_ast(tokens)?;
        assert_eq!(res.run(&mut vec![])?.num(&mut vec![])?, 14.);
        Ok(())
    }

    // #[test]
    // fn test_get_args_from_stack() {
    //     let mut test = vec![Frame::new(), Frame::new(), Frame::new(), Frame::new()];
    //     test[1].set_done();
    //     assert_eq!(get_args_from_stack(&mut test).len(), 2);
    //     assert_eq!(test.len(), 2)
    // }
}
