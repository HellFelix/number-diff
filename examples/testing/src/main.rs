use number_diff::Function;

fn main() {
    let function = Function::from("e^ln(x)");
    println!("{}", function(4.));
}
