use std::f64::consts::PI;

use number_diff::{cos, sinh, Function, X};
fn main() {
    let elem = sinh(cos(X));

    let mut func = Function::new(elem);

    func.differentiate();

    println!("{}", func(1.));
}
