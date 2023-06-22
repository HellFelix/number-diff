use std::f64::consts::PI;

use number_diff::gamma_function;
use number_diff::polygamma_function;
use number_diff::Factorial;
use number_diff::Function;
use number_diff::Integrate;
use number_diff::Round;
use number_diff::EULER_MASCHERONI;

#[test]
fn basic_integration() {
    let function = Function::from("cos(x)");

    // the evaluate_integral() method will automatically round the result to five decimal points.
    // This is because higher precision cannot be guaranteed with using the standard precision set
    // for the method. Provided that the function is defined for all values between the lower and
    // upper bounds, the method will always return a valid result.
    let mut value = function.evaluate_integral(0., PI);

    assert_eq!(value.round_to(10), 0.);
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
    let mut value = integral.evaluate().unwrap();
    // note that the value of the evaluated integral must be unwrapped if using the `integrate()`
    // method because the method cannot guarantee that bounds have been set at the point of
    // evaluating. The evaluate_integral() method which is implemented for any instance with the
    // Integrate trait is safer and is guaranteed to yield a valid result.

    // round the value to 10 decimal places
    value.round_to(10);

    assert_eq!(value, 1.);
}

#[test]
fn gamma_natural() {
    // factorial test
    for i in 0..=34 {
        let correct_answer = i.factorial() as f64;
        // the gamma function gives the correct answer with at least 10 valid significant figures
        // given that x! = 𝜞(x+1)
        assert_eq!(
            gamma_function(i as f64 + 1.).with_significant_figures(10),
            (i.factorial() as f64).with_significant_figures(10)
        )
    }
}

#[test]
fn gamma_float() {
    // Since there is no other function that determines the values that the gamma function takes at
    // non-natural numbers (the gamma function can be evaluated at the natural numbers using the
    // factorial funciton), here, instead, we test some values for the gamma function.
    //
    // Every tested value should be within 5 significant figures.

    const SIG_FIGS: u64 = 5;

    // 𝜞(1.5) = 0.88622692545275801364908374167...
    assert_eq!(
        gamma_function(1.5).with_significant_figures(SIG_FIGS),
        0.88622692545275801364908374167_f64.with_significant_figures(SIG_FIGS)
    );

    // 𝜞(1.7) = 0.9086387328532904499768...
    assert_eq!(
        gamma_function(1.7).with_significant_figures(SIG_FIGS),
        0.9086387328532904499768_f64.with_significant_figures(SIG_FIGS)
    );

    // 𝜞(5.63) = 64.6459979854823802718011...
    assert_eq!(
        gamma_function(5.63).with_significant_figures(SIG_FIGS),
        64.6459979854823802718011_f64.with_significant_figures(SIG_FIGS)
    );

    // 𝜞(20.634) = 8.080423451365037632441106519...*10^17
    assert_eq!(
        gamma_function(20.634).with_significant_figures(SIG_FIGS),
        8.080423451365037632441106519e+17.with_significant_figures(SIG_FIGS)
    );
}

#[test]
fn polygamma() {
    const SIG_FIGS: u64 = 4;

    // 𝝍1(3) = 0.394934066848...
    assert_eq!(
        polygamma_function(3., 1).with_significant_figures(SIG_FIGS),
        0.394934066848_f64.with_significant_figures(SIG_FIGS)
    );

    // 𝝍4(3) = -0.0140631913421
    assert_eq!(
        polygamma_function(5., 4).with_significant_figures(SIG_FIGS),
        -0.0140631913421_f64.with_significant_figures(SIG_FIGS)
    );

    // 𝝍0(1) = -𝛄
    assert_eq!(polygamma_function(1., 0), -EULER_MASCHERONI);

    // 𝝍0(2) = 0.036489973978576520559023667
    assert_eq!(
        polygamma_function(1.5, 0).with_significant_figures(SIG_FIGS),
        0.036489973978576520559023667_f64.with_significant_figures(SIG_FIGS)
    );
}
