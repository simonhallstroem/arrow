use crate::expression::Expression;

#[derive(Clone, Debug)]
pub enum LispType {
    Number(f64),
    String(String),
    Bool(bool),
    Expression(Expression),
    Symbol(String),
    Atom(String, Box<LispType>),
}

impl LispType {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() == 1 {
            if let Ok(n) = args[0].parse::<f64>() {
                return Ok(Self::Number(n));
            } else if args[0].chars().nth(0).unwrap() == '"' {
                return Ok(Self::String(args[0].to_string()));
            } else if args[0] == "t" {
                return Ok(Self::Bool(true));
            } else if args[0] == "nil" {
                return Ok(Self::Bool(false));
            } else {
                return Ok(Self::Expression(Expression::create(
                    args[0].as_str(),
                    vec![],
                )?));
            }
        } else {
            return Err("Not implemented");
        }
        // args.iter().for_each() {
        //     |arg| if arg[1] == ("\"" | "'") {
        // 	ret
        //     }
        // }
    }

    /// Run a [LispType]. This function will return a new
    /// instance of itself (with the same data in it), except
    /// when it is an Expression. Then it will execute the
    /// [Expression].
    ///
    /// # Examples
    /// ```
    /// use arrow::lisptype::LispType;
    /// use arrow::expression::Expression;
    ///
    /// let mut lt = LispType::Expression(Expression::create("*", vec![LispType::Number(2.),
    ///                                                            LispType::Number(2.)]));
    /// assert_eq!(lt.run(&mut vec![])?.num(&mut vec![])?, 4.);
    /// ```    
    pub fn run(&mut self, args: &mut Vec<LispType>) -> Result<Self, &'static str> {
        match self {
            Self::Expression(e) => (*e).run(args),
            Self::Number(n) => Ok(Self::Number(*n)),
            Self::String(s) => Ok(Self::String((*s).clone())),
            Self::Bool(b) => Ok(Self::Bool(*b)),
            Self::Symbol(s) => Ok(Self::Symbol((*s).clone())),
            Self::Atom(_, _) => Err("Cannot return atom!"),
        }
    }

    /// Shortcut to get the [f64] out of the enum.
    /// In every case, except if it is an [f64], it
    /// will fail.
    ///
    /// # Examples
    /// ```
    /// use arrow::lisptype::LispType;
    ///
    /// let lt = LispType::Number(1.);
    ///
    /// assert_eq!(lt.num(&mut vec![])?, 1.);
    /// ```
    pub fn num(&self, vars: &mut Vec<Self>) -> Result<f64, &'static str> {
        match self {
            Self::Number(n) => Ok(*n),
            Self::Symbol(s) => {
                let mut res = 0.;
                let mut flag = true;
                vars.iter().for_each(|n| match n {
                    Self::Atom(a, b) => {
                        if a == s {
                            flag = false;
                            match **b {
                                Self::Number(n) => res = n,
                                _ => panic!("This shouldn't happen."),
                            };
                        }
                    }
                    _ => {}
                });
                if flag {
                    return Err("Didn't find variable with that name.");
                }
                Ok(res)
            }
            _ => Err("Couldn't convert number."),
        }
    }

    /// Shortcut to get the [bool] out of the enum.
    /// If it isn't the field `LispType::Bool`, it could
    /// be that the function will panic.
    ///
    /// # Examples
    /// ```
    /// use arrow::lisptype::LispType;
    ///
    /// let lt = LispType::Number(1.);
    /// let lt_2 = LispType::String("nil".to_string());
    ///
    /// assert_eq!(lt.bool()?, true);
    /// assert_eq!(lt_2.bool()?, false);
    /// ```
    pub fn bool(&self) -> Result<bool, &'static str> {
        match self {
            Self::Bool(b) => Ok(*b),
            Self::String(s) => match s.as_str() {
                "t" => Ok(true),
                "nil" => Ok(false),
                _ => Err("Couldn't convert string to bool."),
            },
            Self::Number(n) => match &n.to_string()[..] {
                "0" => Ok(false),
                "1" => Ok(true),
                _ => Err("Number can't be converted."),
            },
            _ => Err("Couldn't convert to bool."),
        }
    }

    pub fn to_string(&self, vars: &mut Vec<LispType>) -> Result<String, &'static str> {
        match self {
            Self::String(s) => Ok(s.to_string()),
            Self::Number(n) => Ok(n.to_string()),
            Self::Bool(b) => Ok(match b {
                true => "t",
                false => "nil",
            }
            .to_string()),
            Self::Expression(_) => Err("cant convert closure to string."),
            Self::Symbol(s) => {
                let mut res = String::new();
                let mut flag = true;
                vars.iter().for_each(|n| match n {
                    Self::Atom(a, b) => {
                        if a == s {
                            flag = false;
                            res = (*b).to_string(&mut vec![]).unwrap();
                        }
                    }
                    _ => {}
                });
                if flag {
                    res = s.to_string();
                }
                Ok(res)
            }
            Self::Atom(a, b) => Ok(format!("( {} {} )", a, b.to_string(vars)?)),
        }
    }

    pub fn to_string_from_symbol(&self) -> Result<String, &'static str> {
        match self {
            Self::Symbol(s) => Ok(s.to_string()),
            _ => Err("This should only be called if LispType is a Symbol."),
        }
    }
}
