use std::f64::consts::PI;

use number_diff::{cos, sinh, Function, X};
fn main() {
    let func = Function::from("6(4x+3)/5xsin(x)");
    println!("{:?}", func.func);
    println!("{}", func(PI / 2.));
    println!("{}", 6. * (4. * PI / 2. + 3.) / (5. * (PI / 2.)));
}
