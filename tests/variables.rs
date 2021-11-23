use arrow::Arrow;

#[test]
fn test_variable_creation() {
    let test = "(let 'test 10)";
    Arrow::default().add_function(test).unwrap();
}

#[test]
fn test_variable_access() {
    let test = "(defun 'main [] (let 'test 10 (progn (print 'test)))";
    let mut arrow = Arrow::default().add_function(test).unwrap();
    assert_eq!(
        arrow.run("'main").unwrap().to_string(&mut vec![]).unwrap(),
        "10"
    );
}
