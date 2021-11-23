use crate::actaeon::Actaeon;
use crate::lisptype::LispType;
use crate::string::Append;

/// Enum where all functions are registered, that arrow knows about.
#[derive(Clone, Copy, Debug)]
pub enum Func {
    Defun,
    Add,
    Subtract,
    Multiply,
    Concat,
    Equal,
    Print,
    Let,
    Progn,
    Return,
    ActaeonConnect,
    ActaeonReceive,
    ActaeonSend,
}

impl Func {
    /// Create a new function. The argument comes is the name of the arrow function.
    pub fn new(name: &str) -> Result<Self, &'static str> {
        use Func::*;
        match name {
            "defun" => Ok(Defun),
            "+" => Ok(Add),
            "-" => Ok(Subtract),
            "*" => Ok(Multiply),
            "concat" => Ok(Concat),
            "equal" => Ok(Equal),
            "print" => Ok(Print),
            "let" => Ok(Let),
            "progn" => Ok(Progn),
            "return" => Ok(Return),
            "actaeon-create" => Ok(ActaeonConnect),
            "actaeon-receive" => Ok(ActaeonReceive),
            "actaeon-send" => Ok(ActaeonSend),
            _ => Err("invalid argument."),
        }
    }

    /// Function that returns a function pointer, which implements the function that
    /// the arrow function must perform.
    pub fn get_fn(
        &self,
    ) -> Box<dyn Fn(&mut [LispType], &mut Vec<LispType>) -> Result<LispType, &'static str>> {
        use Func::*;
        Box::new(match self {
            Defun => |a: &mut [LispType], v: &mut Vec<LispType>| {
                if a[0].to_string(&mut vec![]).unwrap() == v[0].to_string(&mut vec![]).unwrap() {
                    Ok(LispType::new(
                        &[a.iter_mut()
                            .skip(1)
                            .map(|e| e.run(v).unwrap())
                            .collect::<Vec<LispType>>()
                            .last()
                            .unwrap()
                            .to_string(v)
                            .unwrap()],
                        false,
                    )?)
                } else {
                    Ok(LispType::new(&["'null".to_string()], false).unwrap())
                }
            },
            Add => |a: &mut [LispType], v: &mut Vec<LispType>| {
                Ok(LispType::Number(
                    a[0].run(v)?.num(v)? + a[1].run(v)?.num(v)?,
                ))
            },
            Subtract => |a: &mut [LispType], v: &mut Vec<LispType>| {
                Ok(LispType::Number(
                    a[0].run(v)?.num(v)? - a[1].run(v)?.num(v)?,
                ))
            },
            Multiply => |a: &mut [LispType], v: &mut Vec<LispType>| {
                Ok(LispType::Number(
                    a[0].run(v)?.num(v)? * a[1].run(v)?.num(v)?,
                ))
            },
            Concat => |a: &mut [LispType], v: &mut Vec<LispType>| {
                Ok(LispType::String(
                    a[0].run(v)?
                        .to_string(v)?
                        .append(a[1].run(v)?.to_string(v)?),
                ))
            },
            Equal => |a: &mut [LispType], v: &mut Vec<LispType>| {
                Ok(LispType::Bool(
                    a[0].run(v)?.to_string(v)? == a[1].run(v)?.to_string(v)?,
                ))
            },
            Print => |a: &mut [LispType], v: &mut Vec<LispType>| {
                println!("{}", a[0].run(v)?.to_string(v)?);
                Ok(LispType::Bool(false))
            },
            Let => |a: &mut [LispType], v: &mut Vec<LispType>| {
                v.push(LispType::Atom(
                    a[0].to_string_from_symbol()?,
                    Box::new(a[1].run(&mut vec![])?),
                ));
                let res = a[1].run(v)?;
                v.pop();
                Ok(res)
            },
            Progn => |a: &mut [LispType], v: &mut Vec<LispType>| {
                let mut res = LispType::Bool(false);
                a.iter_mut().for_each(|e| res = e.run(v).unwrap());
                Ok(res)
            },
            Return => |a: &mut [LispType], v: &mut Vec<LispType>| Ok(a[0].run(v)?.clone()),
            ActaeonConnect => |a: &mut [LispType], v: &mut Vec<LispType>| {
                Ok(Actaeon::new(
                    "127.0.0.1",
                    &(a[0].run(v)?.to_string(v)?),
                    4242,
                    &(a[1].run(v)?.to_string(v)?),
                ))
            },
            ActaeonReceive => |a: &mut [LispType], v: &mut Vec<LispType>| {
                if let LispType::Actaeon(mut act) = (&a[0]).clone().run(v)? {
                    Ok(act.receive())
                } else {
                    Err("This is not an acteon type.")
                }
            },
            ActaeonSend => |a: &mut [LispType], v: &mut Vec<LispType>| {
                if let LispType::Actaeon(mut act) = (&a[0]).clone().run(v)? {
                    Ok(act.send(&a[1].run(v)?.to_string(v)?))
                } else {
                    Err("This is not an acteon type.")
                }
            },
        })
    }
}

/// Struct that contains all necessary data (except variables)
/// to execute a LispType.
#[derive(Clone, Debug)]
pub struct Expression {
    pub func: Func,
    pub args: Vec<LispType>,
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
    /// let data = vec![LispType::Number(2.), LispType::Number(3.)];
    /// let mut expr = Expression::create("+", data).unwrap();
    ///
    /// assert_eq!(expr.run(&mut vec![]).unwrap().num(&mut vec![]).unwrap(), 5.);
    /// ```
    pub fn create(name: &str, args: Vec<LispType>) -> Result<Self, &'static str> {
        Ok(Self {
            func: Func::new(name)?,
            args,
        })
    }

    /// Run an instance of the [Expression] struct. It returns the
    /// calculated [LispType].
    ///
    /// # Examples
    /// ```
    /// use arrow::expression::Expression;
    /// use arrow::lisptype::LispType;
    ///
    /// let mut expr = Expression::create(
    ///         "+",
    ///         vec![
    ///             LispType::new(&["1".to_string()], false).unwrap(),
    ///             LispType::new(&["2".to_string()], false).unwrap()
    ///         ]
    /// ).unwrap();
    ///
    /// let res = match expr.run(&mut vec![]).unwrap() {
    ///     LispType::Number(n) => n,
    ///     _ => panic!(""),
    /// };
    /// assert_eq!(res, 3.);
    /// ```
    pub fn run(&mut self, args: &mut Vec<LispType>) -> Result<LispType, &'static str> {
        (*self.func.get_fn())(&mut self.args, args)
    }
}
