use crate::expression::{Expression, Func};
use crate::lisptype::LispType;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct TokenContainer {
    pub name: String,
    pub children: Vec<ChildrenType>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ChildrenType {
    Container(TokenContainer),
    Else(String),
}

impl TokenContainer {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn add_child(&mut self, child: Self) {
        self.children.push(ChildrenType::Container(child));
    }
}

/// Create an ast from a string. After creating the ast, it can be
/// passed into the create_code function to make it executable.
///
/// # Examples
///
/// ```
/// use arrow::tokenize::{ast, TokenContainer, ChildrenType};
///
/// let code = "(+ 4 5)";
/// let exp = TokenContainer {
///     name: "+".to_string(),
///     children: vec![
///         ChildrenType::Else("4".to_string()),
///         ChildrenType::Else("5".to_string()),
///     ]
/// };
///
/// assert_eq!(ast(code)[0], exp);
/// ```
pub fn ast(code: &str) -> Vec<TokenContainer> {
    let mut lasttokenbracket = false;
    let mut working_stack: Vec<TokenContainer> = vec![];

    for token in code
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
    {
        if token == "(" {
            working_stack.push(TokenContainer::default());

            lasttokenbracket = true;
            continue;
        } else if token == ")" {
            let container_done = working_stack.pop().unwrap();
            if let Some(working) = working_stack.last_mut() {
                working.add_child(container_done);
            } else {
                working_stack.push(container_done);
            }
        } else if lasttokenbracket {
            let working = working_stack.last_mut().unwrap();
            working.set_name(token);

            lasttokenbracket = false;
        } else {
            let working = working_stack.last_mut().unwrap();
            working.children.push(ChildrenType::Else(token.to_string()));
        }
    }

    working_stack
}

pub fn create_lisptypes(input: Vec<TokenContainer>) -> Result<Vec<LispType>, &'static str> {
    let mut res: Vec<LispType> = vec![];
    for container in input {
        let mut args: Vec<LispType> = vec![];

        for child in container.children {
            match child {
                ChildrenType::Container(c) => {
                    let lts = create_lisptypes(vec![c])?;

                    for lt in lts {
                        args.push(lt);
                    }
                }
                ChildrenType::Else(e) => args.push(LispType::new(&[e])?),
            }
        }

        res.push(LispType::Expression(Expression {
            func: Func::new(container.name.as_str())?,
            args,
        }));
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_lisptype() {
        let test = "(+ 2 3)";
        assert_eq!(
            create_lisptypes(ast(test)).unwrap()[0]
                .run(&mut vec![])
                .unwrap()
                .num(&mut vec![])
                .unwrap(),
            5.
        );
    }

    #[test]
    fn test_create_lisptype_complex() {
        let test = "(+ (* 4 5) (* 3 (* 6 2)))";
        assert_eq!(
            create_lisptypes(ast(test)).unwrap()[0]
                .run(&mut vec![])
                .unwrap()
                .num(&mut vec![])
                .unwrap(),
            56.
        );
    }

    #[test]
    fn test_create_ast_simple() {
        let test = "(mod 5 2)";
        let test_ast = TokenContainer {
            name: "mod".to_string(),
            children: vec![
                ChildrenType::Else("5".to_string()),
                ChildrenType::Else("2".to_string()),
            ],
        };
        assert_eq!(ast(test)[0], test_ast);
    }

    #[test]
    fn test_create_ast_complex() {
        let test = "(mod 5 (add 4 5))";
        let test_ast = TokenContainer {
            name: "mod".to_string(),
            children: vec![
                ChildrenType::Else("5".to_string()),
                ChildrenType::Container(TokenContainer {
                    name: "add".to_string(),
                    children: vec![
                        ChildrenType::Else("4".to_string()),
                        ChildrenType::Else("5".to_string()),
                    ],
                }),
            ],
        };
        assert_eq!(ast(test)[0], test_ast)
    }
}
