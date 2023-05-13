use number_diff::Function;
fn main() {
    let func = Function::from("4sin(5)+10^ln(x)");
    println!("{:?}", func.func.classify());
}
