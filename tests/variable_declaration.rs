use arrow::expression::Expression;
use arrow::lisptype::LispType;

#[test]
fn test_variables() -> Result<(), &'static str> {
    // The lisp syntax would look the following:
    // (let t 2
    //     (+ t 2))
    let mut test = LispType::Expression(Expression::create(
        "let",
        vec![
            LispType::Symbol("t".to_string()),
            LispType::Number(2.),
            LispType::Expression(Expression::create(
                "+",
                vec![LispType::Symbol("t".to_string()), LispType::Number(2.)],
            )?),
        ],
    )?);

    let res = test.run(&mut vec![])?;

    assert_eq!(res.num(&mut vec![])?, 4.);
    Ok(())
}

#[test]
#[should_panic]
fn test_invalid_scope() {
    // The lisp syntax would look the following:
    // (let t 2
    //     (+ t 2))
    // (+ t 3)
    let mut test = LispType::Expression(
        Expression::create(
            "progn",
            vec![
                LispType::Expression(
                    Expression::create(
                        "let",
                        vec![
                            LispType::Symbol("t".to_string()),
                            LispType::Number(2.),
                            LispType::Expression(
                                Expression::create(
                                    "+",
                                    vec![LispType::Symbol("t".to_string()), LispType::Number(2.)],
                                )
                                .unwrap(),
                            ),
                        ],
                    )
                    .unwrap(),
                ),
                LispType::Expression(
                    Expression::create(
                        "+",
                        vec![LispType::Symbol("t".to_string()), LispType::Number(2.)],
                    )
                    .unwrap(),
                ),
            ],
        )
        .unwrap(),
    );

    test.run(&mut vec![]).unwrap();
}
