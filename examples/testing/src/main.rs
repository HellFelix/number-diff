use number_diff::Function;
fn main() {
    let func = Function::from("4^(6x*sin(x))");
    println!("{:?}", func.func);
    println!("{:?}", func.func.classify());
}
