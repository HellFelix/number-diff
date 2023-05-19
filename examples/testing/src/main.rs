use number_diff::Function;
fn main() {
    let mut func1 = Function::from("4+ 0 + x + 3");
    println!("{:?}", func1.func.simplify())
}
