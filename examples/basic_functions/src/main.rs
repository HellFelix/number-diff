use std::f64::consts::PI;

use number_diff::{abs, cos, sin, Function, X};

fn main() {
    // creating the function by parsing a string, in this case "4sin(x)"
    let func1 = Function::from("4sin(x)");
    assert_eq!(func1.call(PI / 2.), 4.);

    // creating the function by passing an Elementary enum, in this case cos(x)*|sin(x)|
    let func_enum = cos(X) * abs(sin(X));
    let func2 = Function::new(func_enum);
    assert_eq!(func2.call(-PI / 4.), 0.5);
}
