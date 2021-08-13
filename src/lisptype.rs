use crate::expression::Expression;

pub enum LispType {
    Number(f64),
    String(String),
    Bool(bool),
    Expression(Expression),
}

impl LispType {
    pub fn new(args: &[String]) -> Self {
        if args.len() == 1 {
            if let Ok(n) = args[0].parse::<f64>() {
                return Self::Number(n);
            } else if args[0].chars().nth(0).unwrap() == '"' {
                return Self::String(args[0].to_string());
            } else if args[0] == "t" {
                return Self::Bool(true);
            } else if args[0] == "nil" {
                return Self::Bool(false);
            } else {
                return Self::Expression(Expression::create(args[0].as_str(), vec![]));
            }
        } else {
            panic!("Not implemented")
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
    /// assert_eq!(lt.run().num(), 4.);
    /// ```    
    pub fn run(&mut self) -> Self {
        match self {
            Self::Expression(e) => (*e).run(),
            Self::Number(n) => Self::Number((*n).clone()),
            Self::String(s) => Self::String((*s).clone()),
            Self::Bool(b) => Self::Bool((*b).clone()),
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
    /// assert_eq!(lt.num(), 1.);
    /// ```
    pub fn num(&self) -> f64 {
        match self {
            Self::Number(n) => n.clone(),
            _ => panic!("Couldn't convert number."),
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
    /// assert_eq!(lt.bool(), true);
    /// assert_eq!(lt_2.bool(), false);
    /// ```
    pub fn bool(&self) -> bool {
        match self {
            Self::Bool(b) => b.clone(),
	    Self::String(s) => match s.as_str() {
		"t" => true,
		"nil" => false,
		_ => panic!("Couldn't convert string to bool."),
	    },
	    Self::Number(n) => match &n.to_string()[..] {
		"0" => false,
		"1" => true,
		_ => panic!("Number can't be converted."),
	    },
            _ => panic!("Couldn't to bool."),
        }
    }
}

impl ToString for LispType {
    fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.to_string(),
            Self::Number(n) => n.to_string(),
            Self::Bool(b) => match b {
                true => "t",
                false => "nil",
            }
            .to_string(),
            Self::Expression(_) => panic!("cant convert closure to string."),
        }
    }
}
