use std::f64::consts::PI;

use number_diff::{abs, cos, sin, Function};

fn main() {
    // creating the function by parsing a string, in this case "4sin(x)"
    let func1 = Function::from("4sin(x)");
    assert_eq!(func1.call(PI / 2.), 4.);

    // creating the function by passing an Elementary enum, in this case cos(x)*|sin(x)|
    // Function::default() creates an instance of the function's independent variable, in this
    // instance that would be 'x' as described in the expression above
    let func2 = cos(Function::default()) * abs(sin(Function::default()));
    assert_eq!(func2.call(-PI / 4.), 0.5);
}
