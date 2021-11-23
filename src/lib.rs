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
//!     .add_function("(defun 'calc (+ 2 (* 2 3)))").unwrap();
//!
//! assert_eq!(arrow.run("'calc").unwrap().num(&mut vec![]).unwrap(), 8.);
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
//! let mut lisptype = LispType::new(&["12.".to_string()], false).unwrap();
//!
//! assert_eq!(lisptype.num(&mut vec![]).unwrap(), 12.);
//! ```
//!
//! In this example, you can see, that you have to pass an empty vector
//! into the `.num()` function. This is Vector is used if the [LispType]
//! is a `Symbol` and it has to look up, if there is a corresponding
//! variable, the value has to be used.

pub mod actaeon;
pub mod expression;
pub mod lisptype;
pub mod string;
#[cfg(test)]
mod tests;
pub mod tokenize;

use crate::lisptype::LispType;
use crate::tokenize::create_lisptypes;

/// A wrapper struct for this crate.
#[derive(Debug)]
pub struct Arrow {
    funcs: Vec<LispType>,
}

impl Default for Arrow {
    fn default() -> Self {
        Self { funcs: vec![] }
    }
}

impl Arrow {
    /// Add a function to the Crate wrapper struct.
    pub fn add_function(mut self, f: &str) -> Result<Self, &'static str> {
        let tokens = crate::tokenize::ast(f);
        let lisptype = create_lisptypes(vec![tokens[0].clone()])?;
        self.funcs
            .push(lisptype.get(0).ok_or("Invalid input")?.clone());
        Ok(self)
    }

    /// Execute a function, that is registered in the Arrow struct.
    pub fn run(&mut self, n: &str) -> Result<LispType, &'static str> {
        LispType::new(
            &[self
                .funcs
                .iter_mut()
                .map(|e| {
                    e.run(&mut vec![LispType::new(&[n.to_string()], false).unwrap()])
                        .unwrap()
                })
                .collect::<Vec<LispType>>()
                .last()
                .unwrap()
                .to_string(&mut vec![])?],
            false,
        )
    }
}
