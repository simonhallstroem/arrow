use crate::expression::Expression;
use crate::lisptype::LispType;

#[derive(Debug, PartialEq, Clone)]
pub enum Tree {
    Branch(Vec<Self>),
    Leaf(String),
}

impl Tree {
    pub fn new_branch(children: Vec<Self>) -> Self {
        Self::Branch(children)
    }

    pub fn new_leaf(content: &str) -> Self {
        Self::Leaf(content.to_string())
    }
}

/// Create an ast from a string. After creating the ast, it can be
/// passed into the create_code function to make it executable.
///
/// # Examples
///
/// ```
/// use arrow::tokenize::{ast, Tree};
///
/// let code = "(+ 4 5)";
/// let exp = Tree::new_branch(vec![
///     Tree::new_leaf("+"),
///     Tree::new_leaf("4"),
///     Tree::new_leaf("5"),
/// ]);
///
/// assert_eq!(ast(code)[0], exp);
/// ```
pub fn ast(code: &str) -> Vec<Tree> {
    let code = code.replace("(", " ( ").replace(")", " ) ");
    let mut res: Vec<Tree> = vec![];
    let mut leaf_stack: Vec<Tree> = vec![];
    let mut working_token: String = String::new();

    for c in code.chars() {
        match c {
            '(' => {}
            ' ' => {
                if working_token.trim() != "" {
                    leaf_stack.push(Tree::new_leaf(&working_token));
                    working_token = String::new();
                }
            }
            ')' => {
                res.push(Tree::new_branch(leaf_stack.clone()));
                leaf_stack = vec![];
            }
            _ => working_token.push(c),
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ast_simple() {
        let test = "(mod 5 2)";
        let test_ast = vec![Tree::new_branch(vec![
            Tree::new_leaf("mod"),
            Tree::new_leaf("5"),
            Tree::new_leaf("2"),
        ])];
        assert_eq!(ast(test), test_ast)
    }

    #[test]
    fn test_create_ast_complex() {
        let test = "(mod 5 (add 4 5))";
        let test_ast = vec![Tree::new_branch(vec![
            Tree::new_leaf("mod"),
            Tree::new_leaf("5"),
            Tree::new_branch(vec![
                Tree::new_leaf("add"),
                Tree::new_leaf("4"),
                Tree::new_leaf("5"),
            ]),
        ])];
        assert_eq!(ast(test), test_ast)
    }

    #[test]
    fn test_create_ast_complex_2() {
        let test = "(mod (add 4 5)) 5";
        let test_ast = vec![Tree::new_branch(vec![
            Tree::new_leaf("mod"),
            Tree::new_branch(vec![
                Tree::new_leaf("add"),
                Tree::new_leaf("4"),
                Tree::new_leaf("5"),
            ]),
            Tree::new_leaf("5"),
        ])];
        assert_eq!(ast(test), test_ast)
    }
}
