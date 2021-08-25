use crate::expression::Expression;
use crate::lisptype::LispType;

#[test]
fn test_create_defun() -> Result<(), &'static str> {
    let fn_name = "defun";
    let args = vec![LispType::String("nt".to_string()), LispType::Number(22.)];
    let mut expr = Expression::create(fn_name, args)?;
    assert_eq!(
        expr.run(&mut vec![LispType::Symbol("nt".to_string())])?
            .num(&mut vec![])?,
        22.
    );
    Ok(())
}

#[test]
fn test_create_add() -> Result<(), &'static str> {
    let name = "+";
    let args = vec![LispType::Number(33.), LispType::Number(22.)];
    let mut expr = Expression::create(name, args)?;
    assert_eq!(expr.run(&mut vec![])?.num(&mut vec![])?, 55.);
    Ok(())
}

#[test]
fn test_create_multiply() -> Result<(), &'static str> {
    let name = "*";
    let args = vec![LispType::Number(3.), LispType::Number(2.)];
    let mut expr = Expression::create(name, args)?;
    assert_eq!(expr.run(&mut vec![])?.num(&mut vec![])?, 6.);
    Ok(())
}

#[test]
fn test_create_concat() -> Result<(), &'static str> {
    let name = "concat";
    let args = vec![
        LispType::String("h ".to_string()),
        LispType::String("w".to_string()),
    ];
    let mut expr = Expression::create(name, args)?;
    assert_eq!(
        expr.run(&mut vec![])?.to_string(&mut vec![])?,
        "h w".to_string()
    );
    Ok(())
}

#[test]
fn test_create_equal() -> Result<(), &'static str> {
    let name = "equal";
    let args = vec![
        LispType::String("w".to_string()),
        LispType::String("w".to_string()),
    ];
    let mut expr = Expression::create(name, args)?;
    assert!(expr.run(&mut vec![])?.bool()?);
    Ok(())
}

#[test]
fn test_create_print() -> Result<(), &'static str> {
    let name = "print";
    let args = vec![LispType::String("hw".to_string())];
    let mut expr = Expression::create(name, args)?;
    assert!(!expr.run(&mut vec![])?.bool()?);
    Ok(())
}

#[test]
#[should_panic]
fn test_create_fail() {
    let name = "hello";
    let mut expr = Expression::create(name, vec![]).unwrap();
    assert!(!expr.run(&mut vec![]).unwrap().bool().unwrap());
}
