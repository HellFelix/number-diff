use number_diff::{self, Round};

#[test]
fn decimal_rounding() {
    assert_eq!(23.3274.round_to(2), 23.33);

    assert_eq!((1. / 3.).round_to(5), 0.33333);

    // For integer types, rounding to a decimal point is the same as casting it to f64
    assert_eq!(100_u8.round_to(10), 100.);
}

#[test]
fn sig_figs() {
    assert_eq!(14912387964_u128.with_significant_figures(5), 14912000000);

    assert_eq!(-4095_i32.with_significant_figures(1), -4000);

    assert_eq!(1234.5678_f64.with_significant_figures(6), 1234.57);

    assert_eq!(
        0.0000000099934_f64.with_significant_figures(4),
        0.000000009993
    );

    assert_eq!(0.99999999999999999999_f64.with_significant_figures(10), 1.);
}
