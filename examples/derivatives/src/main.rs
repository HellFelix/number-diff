use number_diff::{self, derivative_of, Function};

fn main() {
    // Creating the derivative as a separate Function instance
    // create function
    let sin = Function::from("sin(x)");

    // take derivative
    let sin_derivative = derivative_of(&sin);
    // sin(x)' = cos(x)
    // cos(0) = 1
    assert_eq!(sin_derivative.call(0.), 1.);

    // Casting the Function instance as its derivative
    let mut function = Function::from("cosh(x)");

    // take derivative
    function.differentiate();
    // cosh(x)' = sinh(x)
    // sinh(0) = 0
    assert_eq!(function.call(0.), 0.);

    // Derivatives can also be directly parsed from a string
    let parsed_derivative = Function::from("D(tanh(x))");
    // tanh(x)' = 1/cosh^2(x)
    // 1/cosh^2(0) = 1
    assert_eq!(parsed_derivative.call(0.), 1.);
}
