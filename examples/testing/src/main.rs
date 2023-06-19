use std::{
    f64::{consts::PI, INFINITY},
    time,
};

use number_diff::Function;

fn main() {
    let func = Function::from("e^(-x)").elementary();

    let start = time::SystemTime::now();
    let res = func.integrate(0., 1000. * PI, 10000);
    println!("{}ms", start.elapsed().unwrap().as_millis());

    println!("{res}");
}
