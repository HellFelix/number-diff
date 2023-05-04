use number_diff::{Elementary::*, integrate};
use std::{sync::Arc, f64::consts::E};
fn main() {
    let enum_func = Con(4.);
    let int = integrate(enum_func.call(), 0., 10., 0.1);

    println!("{}", int - 0.218103231563);
}
