//! [Arrow] is a Lisp interpreter library, that can also be compiled
//! into an binary, that has an interactive Shell. It is part of the
//! `Engine: Orion` Project.
//!
//! # Syntax
//!
//! The Syntax mostly resembles the default Lisp implementations. But
//! a lot of function names come from the dialect Emacs Lisp (elisp).
//!
//! ```lisp
//! (defun greet
//!     (print "Hello World!"))
//! ```
//!
//! # Examples
//!
//! ```
//! use arrow::Arrow;
//!
//! let mut arrow = Arrow::default()
//!     .add_function("(defun calc (+ 2 (* 2 3))")
//!     .add_function("(defun subtract (- 2 3))");
//!
//! assert_eq!(arrow.run("calc").num(&mut vec![]), 8.);
//! assert_eq!(arrow.run("subtract").num(&mut vec![]), -1.);
//! ```
//!
//! # Caveats
//!
//! If you are using this library in your own project. You have to
//! understand, that sometimes, you are using the same functions, that
//! the library is using internally. This has mostly the effect, that
//! you often have to pass mutable references to Vectors, because these
//! are internally used to pass lisp variables.
//!
//! ```
//! use arrow::lisptype::LispType;
//!
//! let mut lisptype = LispType::new(&["12.".to_string()]);
//!
//! assert_eq!(lisptype.num(&mut vec![]), 12.);
//! ```
//!
//! In this example, you can see, that you have to pass an empty vector
//! into the `.num()` function. This is Vector is used if the [LispType]
//! is a `Symbol` and it has to look up, if there is a corresponding
//! variable, the value has to be used.

pub mod expression;
pub mod lisptype;
pub mod string;
pub mod tokenize;
#[cfg(test)]
mod tests;

use crate::lisptype::LispType;

/// A wrapper struct for this crate.
pub struct Arrow {
    funcs: Vec<LispType>,
}

impl Default for Arrow {
    fn default() -> Self {
        Self { funcs: vec![] }
    }
}

impl Arrow {
    pub fn add_function(mut self, f: &str) -> Self {
        let tokens = crate::tokenize::tokenize(f);
        let ast = crate::tokenize::create_ast(tokens);
        self.funcs.push(ast);
        self
    }

    pub fn run(&mut self, n: &str) -> LispType {
        LispType::new(&[self
            .funcs
            .iter_mut()
            .map(|e| e.run(&mut vec![LispType::Symbol(n.to_string())]))
            .collect::<Vec<LispType>>()
            .last()
            .unwrap()
            .to_string(&mut vec![])])
    }
}
