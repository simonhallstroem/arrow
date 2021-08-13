use arrow::expression::Expression;
use arrow::lisptype::LispType;

#[test]
fn test_simple_example_f() {
    let mut test = LispType::Number(12.);
    let exp = 12.;
    assert_eq!(test.run().num(), exp);
}

#[test]
fn test_simple_example_str() {
    let mut test = LispType::String("Hello".to_string());
    let exp = "Hello".to_string();
    assert_eq!(test.run().to_string(), exp);
}

#[test]
fn test_advanced_example() {
    let mut test = LispType::Expression(Expression::create(
        "+",
        vec![
            LispType::Expression(Expression::create(
                "+",
                vec![LispType::Number(12.), LispType::Number(3.)],
            )),
            LispType::Number(22.),
        ],
    ));
    let exp = 37.;
    assert_eq!(test.run().num(), exp);
}
