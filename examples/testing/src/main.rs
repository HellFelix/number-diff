use number_diff::Function;

fn main() {
    let function = Function::from("sin(ln(e))");
    println!("{}", function.call(4.));
}
