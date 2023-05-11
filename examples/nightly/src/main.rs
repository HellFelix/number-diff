use number_diff::test_parse;
fn main() {
    let binding = String::from("sin(exp(x)) + cos(x)");
    let res = test_parse(&binding);

    println!("{res:?}");
}
