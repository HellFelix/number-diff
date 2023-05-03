use number_diff::{Elementary::*, integrate};
use std::{sync::Arc, f64::consts::E};
fn main() {
    let enum_func = Sin(Arc::new(Pow(Arc::new(X), Arc::new(Con(3.)))));
    let int = integrate(enum_func.call(), -1., 2., 0.0000001);

    println!("{}", int - 0.218103231563);
}
