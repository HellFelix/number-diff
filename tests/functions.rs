use std::f64::consts::PI;

use number_diff::{Elementary::*, Function, Round, EULER_MASCHERONI};

#[test]
fn parsing() {
    // let sin = Function::from("sin(x)");

    // assert!(sin.call(PI) < 1e-15);

    // let pol_string = String::from("3x^4 + 9x^3 - 3x^2 - 14x");
    // let polynomial = Function::from(pol_string);
    // assert_eq!(polynomial.call(3.), 417.);

    // let const_157 = Function::from("1234");
    // assert_eq!(const_157.call(1.), 1234.);

    let factorial = Function::from("x!");
    assert_eq!(factorial.call(5.), 120.);
}

#[test]
fn gamma_function() {
    let gamma = Gamma(X.into());

    const SIG_FIGS: u64 = 4;

    // 𝜞(1.5) = 0.88622692545275801364908374167...
    assert_eq!(
        gamma.clone().call()(1.5).with_significant_figures(SIG_FIGS),
        0.88622692545275801364908374167_f64.with_significant_figures(SIG_FIGS)
    );

    // 𝜞(1.7) = 0.9086387328532904499768...
    assert_eq!(
        gamma.clone().call()(1.7).with_significant_figures(SIG_FIGS),
        0.9086387328532904499768_f64.with_significant_figures(SIG_FIGS)
    );

    // 𝜞'(3) = 1.8455686701969342787
    assert_eq!(
        gamma.clone().derivative_unsimplified().call()(3.).with_significant_figures(SIG_FIGS),
        1.8455686701969342787_f64.with_significant_figures(SIG_FIGS)
    );

    // 𝜞⁽³⁾(2.7) = 2.12832
    assert_eq!(
        gamma
            .clone()
            .derivative_unsimplified()
            .derivative_unsimplified()
            .derivative_unsimplified()
            .call()(2.7)
        .with_significant_figures(SIG_FIGS),
        2.12832_f64.with_significant_figures(SIG_FIGS)
    );
    // it seems that taking derivatives of the gamma function does not decrease its accuracy.

    // d²/dx²(𝜞(sinh(x))) ⎮x=1.3 = 3.27363
    let gamma_sinh = Gamma(Sinh(X.into()).into());
    assert_eq!(
        gamma_sinh
            .derivative_unsimplified()
            .derivative_unsimplified()
            .call()(1.3)
        .with_significant_figures(3),
        3.27363.with_significant_figures(3)
    );

    let gamma_atan = Gamma(Atan(X.into()).into());
    assert_eq!(
        gamma_atan
            .derivative_unsimplified()
            .derivative_unsimplified()
            .derivative_unsimplified()
            .call()(8.)
        .with_significant_figures(2),
        -0.000173257_f64.with_significant_figures(2)
    );
    // It seems like taking other functions as arguments for the gamma function might lower the
    // accuracy.
}

#[test]
fn polygamma_function() {
    let digamma = Polygamma(X.into(), 0);

    const SIG_FIGS: u64 = 5;

    assert_eq!(
        digamma.clone().call()(1.).with_significant_figures(SIG_FIGS),
        -EULER_MASCHERONI.with_significant_figures(SIG_FIGS)
    );

    assert_eq!(
        digamma.clone().call()(3.).with_significant_figures(SIG_FIGS),
        0.9227843350984671393_f64.with_significant_figures(SIG_FIGS)
    );
}

#[test]
fn factorial() {
    let factorial = Factorial(X.into());

    const SIG_FIGS: u64 = 8;

    // integer values
    assert_eq!(factorial.clone().call()(0.), 1.);
    assert_eq!(factorial.clone().call()(1.), 1.);
    assert_eq!(factorial.clone().call()(2.), 2.);
    assert_eq!(factorial.clone().call()(6.), 720.);

    // non-integer values
    assert_eq!(
        factorial.clone().call()(1.5).with_significant_figures(SIG_FIGS),
        1.32934038817913702047362561_f64.with_significant_figures(SIG_FIGS)
    );

    assert_eq!(
        factorial.clone().call()(0.2).with_significant_figures(3),
        0.9181687423997606106409_f64.with_significant_figures(3)
    );
    // it seems that for smaller valus of the factorial function, the accuracy decreases (just as
    // is for the gamma function wich tracks)

    assert_eq!(
        factorial.clone().derivative_unsimplified().call()(5.14).with_significant_figures(4),
        264.265.with_significant_figures(4)
    );

    assert_eq!(
        factorial.clone().derivative_unsimplified().call()(5.).with_significant_figures(6),
        204.734
    );

    let cosh_factorial = Factorial(Cosh(X.into()).into());

    assert_eq!(
        cosh_factorial.clone().call()(0.9).with_significant_figures(SIG_FIGS),
        1.269650765516591624581854_f64.with_significant_figures(SIG_FIGS)
    );
}
