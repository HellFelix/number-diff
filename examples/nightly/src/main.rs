use std::f64::consts::{E, PI};

use number_diff::Function;
fn main() {
    // create function instance
    let func = Function::from("sin(x)");

    // using nightly feature allows funciton instances to be callable
    // here we're calling the funciton instance with the argument of π/2
    assert_eq!(func(PI / 2.), 1.);
}
