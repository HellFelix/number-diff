use std::f64::consts::PI;

use number_diff::{cos, sinh, Function, X};
fn main() {
    let func = Function::from("sin(x)");
    println!("{}", func(PI / 2.));
}
