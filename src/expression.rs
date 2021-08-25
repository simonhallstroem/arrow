use crate::lisptype::LispType;
use crate::string::Append;

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
}

impl Func {
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
            _ => Err("invalid argument."),
        }
    }

    pub fn get_fn(
        &self,
    ) -> Box<dyn Fn(&mut [LispType], &mut Vec<LispType>) -> Result<LispType, &'static str>> {
        use Func::*;
        Box::new(match self {
            Defun => |a: &mut [LispType], v: &mut Vec<LispType>| {
                if a[0].to_string(&mut vec![]).unwrap() == v[0].to_string(&mut vec![]).unwrap() {
                    Ok(LispType::new(&[a
                        .iter_mut()
                        .skip(1)
                        .map(|e| e.run(v).unwrap())
                        .collect::<Vec<LispType>>()
                        .last()
                        .unwrap()
                        .to_string(v)
                        .unwrap()])?)
                } else {
                    Ok(LispType::Bool(false))
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
                let res = a[2].run(v)?;
                v.pop();
                Ok(res)
            },
            Progn => |a: &mut [LispType], v: &mut Vec<LispType>| {
                let mut res = LispType::Bool(false);
                a.iter_mut().for_each(|e| res = e.run(v).unwrap());
                Ok(res)
            },
        })
    }
}

#[derive(Clone, Debug)]
pub struct Expression {
    func: Func,
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
    /// let data = vec![LispType::Number(2.), LispType::Number(3.)];
    /// let mut expr = Expression::create("+", data);
    ///
    /// assert_eq!(expr.run(&mut vec![])?.num(&mut vec![])?, 5.);
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
    /// let mut expr = Expression::create("+", vec![LispType::new(&["1".to_string()]),
    ///                                             LispType::new(&["2".to_string()])]);
    /// let res = match expr.run(&mut vec![])? {
    ///     LispType::Number(n) => n,
    ///     _ => panic!(""),
    /// };
    /// assert_eq!(res, 3.);
    /// ```
    pub fn run(&mut self, args: &mut Vec<LispType>) -> Result<LispType, &'static str> {
        (*self.func.get_fn())(&mut self.args, args)
    }
}
