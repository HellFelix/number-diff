use std::time;

use number_diff::Function;

fn main() {
    let func = Function::from("sin(x)");
    let start = time::SystemTime::now();
    let _integral = func.elementary().integrate(0., 10., 5);
    println!(
        "entire integration: {}ms",
        start.elapsed().unwrap().as_millis()
    );
    println!("{_integral:?}");
}
