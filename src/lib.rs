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

    pub fn run(&mut self) -> Self {
        match self {
            Self::Expression(e) => (*e).run(),
            Self::Number(n) => Self::Number((*n).clone()),
            Self::String(s) => Self::String((*s).clone()),
            Self::Bool(b) => Self::Bool((*b).clone()),
        }
    }

    pub fn num(&self) -> f64 {
        match self {
            Self::Number(n) => n.clone(),
            _ => panic!("Couldn't convert number."),
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

pub struct Expression {
    func: Box<dyn Fn(&mut [LispType]) -> LispType>,
    args: Vec<LispType>,
}

impl Expression {
    fn new(func: Box<dyn Fn(&mut [LispType]) -> LispType>, args: Vec<LispType>) -> Self {
        Self {
            func: Box::new(func),
            args,
        }
    }

    pub fn create(name: &str, arg: Vec<LispType>) -> Self {
        match name {
            "+" => Self::new(
                Box::new(|a: &mut [LispType]| {
                    LispType::Number(a[0].run().num() + a[1].run().num())
                }),
                arg,
            ),
            "=" => Self::new(
                Box::new(|a: &mut [LispType]| {
                    LispType::Bool(a[0].run().to_string() == a[1].run().to_string())
                }),
                arg,
            ),
            "print" => Self::new(
                Box::new(|a: &mut [LispType]| {
                    println!("{}", a[0].to_string());
                    LispType::Bool(false)
                }),
                arg,
            ),
            _ => panic!("invalid fn name!"),
        }
    }

    /// Run an instance of the [Expression] struct. It returns the
    /// calculated [LispType].
    ///
    /// # Examples
    /// ```
    /// use arrow::Expression;
    /// use arrow::LispType;
    ///
    /// let mut expr = Expression::create("+", vec![LispType::new(&["1".to_string()]),
    ///	       					    LispType::new(&["2".to_string()])]);
    /// let res = match expr.run() {
    ///     LispType::Number(n) => n,
    ///     _ => panic!(""),
    /// };
    /// assert_eq!(res, 3.);
    /// ```
    pub fn run(&mut self) -> LispType {
        (*self.func)(&mut self.args)
    }
}
