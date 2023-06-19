use std::f64::{consts::E, INFINITY, NAN};

use crate::{Elementary::*, Function, Integrate};

pub const COMPLEX_INFINITY: f64 = NAN;

/// returns n! for numbers n ∈ ℕ
fn factorial_integer(numb: usize) -> usize {
    if numb == 0 {
        1
    } else {
        let mut res: usize = 1;
        for i in 1..=numb {
            res *= i;
        }

        res
    }
}

pub fn gamma_function(z: f64) -> f64 {
    let inner_funciton = Mul(
        Pow(X.into(), Sub(Con(z).into(), Con(1.).into()).into()).into(),
        Pow(Con(E).into(), Mul(X.into(), Con(-1.).into()).into()).into(),
    );

    inner_funciton.evaluate_integral(0., 100.)
}

/// Allows the usage of factorials i.e. `x!`
/// usually defined as:
/// * x! = x*(x-1)!
/// * 0! = 1
///
/// for x ∈ ℝ.
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
            type Output = usize;
            fn factorial(&self) -> Self::Output {
                factorial_integer(self.clone() as usize)
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
                    factorial_integer(self.clone() as usize) as f64
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
                unimplemented!()
            }
        })*
    }
}
impl_factorial_float!(for f32, f64);

pub trait Round {
    fn round_to(&mut self, decimal_places: u32) -> f64;
    fn with_significant_figures(&mut self, digits: usize) -> Self;
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

            fn with_significant_figures(&mut self, digits: usize) -> Self {
                let value = if *self > 0 as Self {
                    unimplemented!()
                } else if *self == 0 as Self {
                    0 as Self
                } else {
                    (-1 as Self) * (-1 as Self * self.clone()).with_significant_figures(digits)
                };

                *self = value as Self;
                value
            }
        })*
    };
}
impl_round_float!(for f32, f64);
