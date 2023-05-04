use number_diff::{Elementary::*, integrate, Function};
use std::{sync::Arc, f64::consts::E};
fn main() {
    let func = Function::new(X);

    println!("{}", func(5.));
}
