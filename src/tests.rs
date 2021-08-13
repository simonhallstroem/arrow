use crate::expression::Expression;
use crate::lisptype::LispType;

#[test]
fn test_create_defun() {
    let fn_name = "defun";
    let args = vec![LispType::String("nt".to_string()),
		    LispType::Number(22.)];
    let mut expr = Expression::create(fn_name, args);
    assert_eq!(expr.run().num(), 22.);
}

#[test]
fn test_create_add() {
    let name = "+";
    let args = vec![LispType::Number(33.),
		    LispType::Number(22.)];
    let mut expr = Expression::create(name, args);
    assert_eq!(expr.run().num(), 55.);
}

#[test]
fn test_create_multiply() {
    let name = "*";
    let args = vec![LispType::Number(3.),
		    LispType::Number(2.)];
    let mut expr = Expression::create(name, args);
    assert_eq!(expr.run().num(), 6.);
}

#[test]
fn test_create_concat() {
    let name = "concat";
    let args = vec![LispType::String("h ".to_string()),
		    LispType::String("w".to_string())];
    let mut expr = Expression::create(name, args);
    assert_eq!(expr.run().to_string(), "h w".to_string());
}

#[test]
fn test_create_equal() {
    let name = "equal";
    let args = vec![LispType::String("w".to_string()),
		    LispType::String("w".to_string())];
    let mut expr = Expression::create(name, args);
    assert_eq!(expr.run().bool(), true);
}

#[test]
fn test_create_print() {
    let name = "print";
    let args = vec![LispType::String("hw".to_string())];
    let mut expr = Expression::create(name, args);
    assert_eq!(expr.run().bool(), false);
}

#[test]
#[should_panic]
fn test_create_fail() {
    let name = "hello";
    let mut expr = Expression::create(name, vec![]);
    assert_eq!(expr.run().bool(), false);
}
