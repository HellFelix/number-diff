use std::f64::consts::PI;

use number_diff::Function;
use number_diff::Integrate;

#[test]
fn basic_integration() {
    let function = Function::from("cos(x)");

    // the evaluate_integral() method will automatically round the result to five decimal points.
    // This is because higher precision cannot be guaranteed with using the standard precision set
    // for the method. Provided that the function is defined for all values between the lower and
    // upper bounds, the method will always return a valid result.
    let value = function.evaluate_integral(0., PI);

    assert_eq!(value, 0.);
}

#[test]
fn specified_precision_integration() {
    let function = Function::from("sin(x)");

    let mut integral = function.integrate();

    // specify bounds and precision for the integral
    integral
        .set_lower_bound(0.)
        .set_upper_bound(PI / 2.)
        .set_precision(20000);

    // evaluate the integral
    let value = integral.evaluate().unwrap();
    // note that the value of the evaluated integral must be unwrapped if using the `integrate()`
    // method because the method cannot guarantee that bounds have been set at the point of
    // evaluating. The evaluate_integral() method which is implemented for any instance with the
    // Integrate trait is safer and is guaranteed to yield a valid result.

    // round the value to 5 decimal places
    let rounded_value = (value * 100000.).round() / 100000.;

    assert_eq!(rounded_value, 1.);
}
