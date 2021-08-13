use crate::lisptype::LispType;
use crate::string::Append;

pub struct Expression {
    func: Box<dyn Fn(&mut [LispType]) -> LispType>,
    args: Vec<LispType>,
}

impl Expression {
    /// Create a new instance of the [Expression] struct. Please
    /// usually use the associated `create` method.
    ///
    /// # Example
    /// ```
    /// use arrow::expression::Expression;
    /// use arrow::lisptype::LispType;
    ///
    /// let fun = Box::new(|a: &mut [LispType]| LispType::String(a[0].to_string()));
    /// let data = vec![LispType::String("hw".to_string())];
    /// let mut expr = Expression::new(fun, data);
    ///
    /// assert_eq!(expr.run().to_string(), "hw".to_string());
    /// ```
    pub fn new(func: Box<dyn Fn(&mut [LispType]) -> LispType>, args: Vec<LispType>) -> Self {
        Self {
            func: Box::new(func),
            args,
        }
    }

    /// Create an new instance of the [Expression] struct. The first
    /// parameter is the name of the function. The next is is a
    /// [Vec<String>] with all the parameters. The parameters have
    /// to be already converted into a LispType. That means, that
    /// can be also be an Expression.
    ///
    /// # Examples
    /// ```
    /// use arrow::expression::Expression;
    /// use arrow::lisptype::LispType;
    ///
    /// let mut expr = Expression::create("concat", vec![LispType::String("h".to_string()),
    ///                                                  LispType::String("w".to_string())]);
    /// assert_eq!(expr.run().to_string(), "hw".to_string());
    /// ```
    pub fn create(name: &str, arg: Vec<LispType>) -> Self {
        match name {
            "defun" => Self::new(
                Box::new(|a: &mut [LispType]| {
                    LispType::new(&[a
                        .iter_mut()
                        .skip(1)
                        .map(|e| e.run())
                        .collect::<Vec<LispType>>()
                        .last()
                        .unwrap()
                        .to_string()])
                }),
                arg,
            ),
            "+" => Self::new(
                Box::new(|a: &mut [LispType]| {
                    LispType::Number(a[0].run().num() + a[1].run().num())
                }),
                arg,
            ),
            "*" => Self::new(
                Box::new(|a: &mut [LispType]| {
                    LispType::Number(a[0].run().num() * a[1].run().num())
                }),
                arg,
            ),
            "concat" => Self::new(
                Box::new(|a: &mut [LispType]| {
                    LispType::String(a[0].run().to_string().append(a[1].run().to_string()))
                }),
                arg,
            ),
            "equal" => Self::new(
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
    /// use arrow::expression::Expression;
    /// use arrow::lisptype::LispType;
    ///
    /// let mut expr = Expression::create("+", vec![LispType::new(&["1".to_string()]),
    ///                                             LispType::new(&["2".to_string()])]);
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
