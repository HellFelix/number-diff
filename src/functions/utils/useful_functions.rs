use std::f64::{consts::E, NAN};

use crate::{Elementary::*, Integrate, EULER_MASCHERONI};

/// An infinit number in the complex plane with an unknown or undefined complex argument.
///
/// For instance will approach complex infinity as x approaches 0. The limit could be either plus
/// or minus infinity wich makes its magnitude infinit and its complex argument undefined.
///
/// See [this article](https://mathworld.wolfram.com/ComplexInfinity.html) for further information.
pub const COMPLEX_INFINITY: f64 = NAN;

/// returns n! for numbers n ∈ ℕ
fn factorial_integer(numb: u128) -> u128 {
    if numb == 0 {
        1
    } else {
        let mut res: u128 = 1;
        for i in 1..=numb {
            res *= i;
        }

        res
    }
}

// TODO: make the gamma function work for values < 1
/// Returns the value of 𝜞(z) as defined by ∫t^(z-1)e^(-t)dt evaluated from 0 to ∞.
pub fn gamma_function(z: f64) -> f64 {
    let inner_funciton = Mul(
        Pow(X.into(), Sub(Con(z).into(), Con(1.).into()).into()).into(),
        Pow(Con(E).into(), Mul(X.into(), Con(-1.).into()).into()).into(),
    );

    // for whole numbers
    if z.fract() == 0.0 {
        // fraction part of the number is zero, meaning that the number is an integer
        inner_funciton
            .integrate()
            .set_lower_bound(0.)
            .set_upper_bound(100.)
            .set_precision(1000)
            .evaluate()
            .unwrap()
            .round_to(0)
    } else {
        inner_funciton
            .integrate()
            .set_lower_bound(0.)
            .set_upper_bound(100.)
            .set_precision(100000)
            .evaluate()
            .unwrap()
    }
}

/// the polygamma function 𝛙m(z) describes the relationship between 𝜞(z) and its derivatives. For instance 𝛙0(z) =
/// 𝜞'(z)/𝜞(z). [See article](https://en.wikipedia.org/wiki/Polygamma_function)
pub fn polygamma_function(z: f64, m: usize) -> f64 {
    if m == 0 {
        digamma_function(z)
    } else {
        let inner_funciton = Pow(Log(Con(E).into(), X.into()).into(), Con(m as f64).into())
            * Pow(X.into(), Con(z - 1.).into())
            / (Con(1.) - X);

        -inner_funciton
            .integrate()
            .set_lower_bound(1e-10)
            .set_upper_bound(1. - 1e-10)
            .set_precision(10)
            .evaluate()
            .unwrap()
    }
}

/// Special case of the polygamma function 𝛙m(z) where m=0, The integral definition of the function
/// then changes. [See article](https://en.wikipedia.org/wiki/Digamma_function#Integral_representations)
pub fn digamma_function(z: f64) -> f64 {
    let inner_funciton = (Con(1.) - Pow(X.into(), Con(z - 1.).into())) / (Con(1.) - X);

    let integral_value = inner_funciton
        .integrate()
        .set_lower_bound(0.)
        .set_upper_bound(1. - 1e-10)
        .set_precision(1000)
        .evaluate()
        .unwrap();

    integral_value - EULER_MASCHERONI
}

/// Allows the usage of factorials i.e. `x!`
/// usually defined as:
/// * x! = x*(x-1)!
/// * 0! = 1
///
/// for x ∈ ℕ.
///
/// The definition of the factorial function can be expanded to the domain of all real numbers
/// using the [gamma function](https://en.wikipedia.org/wiki/Gamma_function):
///
/// x! = 𝜞(x+1)
///
pub trait Factorial {
    type Output;
    fn factorial(&self) -> Self::Output;
}

/// implement factorial method for natural number types
macro_rules! impl_factorial_natural {
    (for $($t:ty), +) => {
        $(impl Factorial for $t {
            type Output = u128;
            fn factorial(&self) -> Self::Output {
                factorial_integer(self.clone() as u128)
            }
        })*
    };
}
impl_factorial_natural!(for u8, u16, u32, u64, u128, usize);

/// implement factorial for integer number types
macro_rules! impl_factorial_integer {
    (for $($t:ty), +) => {
        $(impl Factorial for $t {
            type Output = f64;
            fn factorial(&self) -> Self::Output {
                if self.is_negative() {
                    // the continuos factorial funciton approaches ±∞ (complex infinity) for any
                    // negative integer
                    NAN
                } else {
                    factorial_integer(self.clone() as u128) as f64
                }
            }
        })*
    };
}
impl_factorial_integer!(for i8, i16, i32, i64, i128, isize);

/// implement factorial method for float types
macro_rules! impl_factorial_float {
    (for $($t:ty), +) => {
        $(impl Factorial for $t {
            type Output = Self;
            fn factorial(&self) -> Self::Output{
                gamma_function(*self as f64 + 1.) as Self
            }
        })*
    }
}
impl_factorial_float!(for f32, f64);

/// Allows the usage of rounding methods that are more specific than rust std's round() method.
pub trait Round {
    /// Rounds self (a number) to the given number of decimal places. This method is
    /// mainly made for the f32 and f64 types since integer types already have no decimal places.
    ///
    /// Example:
    /// ```rust
    /// assert_eq!(23.3274.round_to(2), 23.33);
    ///
    /// assert_eq!((1. / 3.).round_to(5), 0.33333);
    ///
    /// // For integer types, rounding to a decimal point is the same as casting it to f64
    /// assert_eq!(100_u8.round_to(10), 100.);
    /// ```
    fn round_to(&mut self, decimal_places: u32) -> f64;

    /// Rounds self (a number) to the given number of significant figures.
    /// Example:
    /// ```rust
    /// assert_eq!(14912387964_u128.with_significant_figures(5), 14912000000);
    ///
    /// assert_eq!(-4095_i32.with_significant_figures(1), -4000);
    ///
    /// assert_eq!(1234.5678_f64.with_significant_figures(6), 1234.57)
    /// ```
    fn with_significant_figures(&mut self, digits: u64) -> Self;
}

macro_rules! impl_round_float {
    (for $($t:ty), +) => {
        $(impl Round for $t {
            fn round_to(&mut self, decimal_places: u32) -> f64 {
                let mut value = *self as f64;
                // move the decimal point
                value *= 10_usize.pow(decimal_places) as f64;
                // round to an integer
                value = value.round();
                // move the decimal point back
                value /= 10_usize.pow(decimal_places) as f64;
                // give self the value of the rounded number
                *self = value as Self;
                value
            }

            fn with_significant_figures(&mut self, digits: u64) -> Self {
                let value = if *self >= 0. {

                    let order = (*self).log10().trunc() as i32;
                        if digits as i32 <= order {
                            ((*self) as isize).with_significant_figures(digits) as Self
                        } else {
                            if *self >= 1. {
                                (*self * (10 as Self).powi((digits as i32 - order -1) as i32)).round() / (10 as Self).powi((digits as i32 - order -1) as i32)
                            } else {
                                (*self * (10 as Self).powi((digits as i32 - order) as i32)).round() / (10 as Self).powi((digits as i32 - order) as i32)
                            }
                        }
                } else {
                    -1. *(*self *-1.).with_significant_figures(digits)
                };

                *self = value as Self;
                value
            }
        })*
    };
}
impl_round_float!(for f32, f64);

macro_rules! impl_round_int {
    (for $($t:ty), +) => {
        $(impl Round for $t {
            #[allow(unused_variables)]
            fn round_to(&mut self, decimal_places: u32) -> f64 {
                *self as f64
            }
            fn with_significant_figures(&mut self, digits: u64) -> Self {
                // move the decimal point to the appropriate spot so that we can round and then
                // move it back
                let order = (*self).ilog10() as u64;
                let new_value = ((*self) as f64 / 10_f64.powi((order - digits +1) as i32)).round() * 10_f64.powi((order - digits +1) as i32);
                // set new value
                *self = new_value as Self;
                *self
            }
        })*

    };
}
impl_round_int!(for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
