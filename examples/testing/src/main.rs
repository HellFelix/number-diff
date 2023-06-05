use std::f64::consts::PI;

use number_diff::{derivative_of, sin, Elementary::*, Function};

fn main() {
    let func = Function::from("sin(x)");

    // Get the SeriesExpansion
    // In this instance we're creating a Taylor expansion of order 5 centered around 0
    let expansion = func.get_taylor_expansion(5, 0.).unwrap();

    let func_expansion = Function::from(expansion);
    let element = func_expansion.elementary();
    let derivative = derivative_of(&func_expansion);
    println!("{:?}", derivative.elementary());
}
