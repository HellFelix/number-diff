use number_diff::{sin, Function};

fn main() {
    // create the Function instance
    let func = Function::from("sin(x)");

    // Get the SeriesExpansion
    // In this instance we're creating a Taylor expansion of order 5 centered around 0
    let expansion = func.get_taylor_expansion(3, 0.).unwrap();

    // Convert the SeriesExpansion into a Function using the from method
    let mut func_expansion = Function::from(expansion);
    // Note that this could also be done using the get_function method:
    // let func_expansion = expansion.get_function()
    //
    // ... or using the as_taylor_expansion method to convert the original Function instance into a
    // Taylor expansion without creating the SeriesExpansion instance seperatly:
    // func.as_taylor_expansion()

    let mut sin = sin(Function::default());

    assert_eq!(func_expansion.call(0.), sin.call(0.));

    println!("{:?}", func_expansion.elementary().classify());

    // first derivative
    func_expansion.differentiate();
    sin.differentiate();

    assert_eq!(func_expansion.call(0.), sin.call(0.));

    // second derivative
    func_expansion.differentiate();
    sin.differentiate();

    assert_eq!(func_expansion.call(0.), sin.call(0.));

    // third derivative
    func_expansion.differentiate();
    sin.differentiate();

    assert_eq!(func_expansion.call(0.), sin.call(0.));
}
