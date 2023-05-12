use number_diff::test_parse;
fn main() {
    let binding = String::from("x*x^sin(x)*x");
    let res = test_parse(&binding);

    println!("{res:?}");
}
