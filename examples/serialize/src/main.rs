use number_diff::Function;
fn main() {
    // initialize a function instance
    let function = Function::from("sin(cos(x^x + 3))");

    // serialize to json string
    let json = serde_json::to_string(&function).unwrap();

    // the outcome will be a Elementary representation of the function in json format
    let expected_json =
        r#"{"Elementary":{"Sin":{"Cos":{"Add":[{"Con":3.0},{"Pow":["X: {}","X: {}"]}]}}}}"#;

    assert_eq!(json, expected_json);
}
