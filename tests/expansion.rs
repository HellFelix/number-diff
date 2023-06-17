use std::time;

use number_diff::{self, Function};

#[test]
fn ensure_equality() {
    let sin = Function::from("sin(x)");
    for i in 0..1 {
        let start = time::SystemTime::now();
        let expansion = sin.get_taylor_expansion(5, i as f64);
        println!("{}", start.elapsed().unwrap().as_secs());
    }
}
