use std::f64::consts::PI;

use number_diff::Function;

#[test]
fn parsing() {
    let sin = Function::from("sin(x)");

    assert!(sin.call(PI) < 1e-15);

    let pol_string = String::from("3x^4 + 9x^3 - 3x^2 - 14x");
    let polynomial = Function::from(pol_string);
    assert_eq!(polynomial.call(3.), 417.);

    let factorial = Function::from("x!");
    assert_eq!(factorial.call(5.), 120.);
}
