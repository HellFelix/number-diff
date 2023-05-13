use crate::Elementary::*;
use std::{
    f64::consts::E,
    ops::{Add, Div, Mul, Sub},
    sync::Arc,
};

use crate::Func;

// unit function f(x) -> x
fn f() -> Func {
    Box::new(move |x| x)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Elementary {
    // Standard trig functions
    Sin(Arc<Elementary>), // of the type sin(f(x))
    Cos(Arc<Elementary>), // of the type cos(f(x))
    Tan(Arc<Elementary>), // of the type tan(f(x))

    // Standard arcus functions
    Asin(Arc<Elementary>), // of the type arcsin(f(x))
    Acos(Arc<Elementary>), // of the type arccos(f(x))
    Atan(Arc<Elementary>), // of the type arctan(f(x))

    Sinh(Arc<Elementary>), // of the type sinh(f(x))
    Cosh(Arc<Elementary>), // of the type cosh(f(x))
    Tanh(Arc<Elementary>), // of the type tanh(f(x))

    // Standard operations
    Add(Arc<Elementary>, Arc<Elementary>), // of the type f(x) + g(x)
    Sub(Arc<Elementary>, Arc<Elementary>), // of the type f(x) - g(x)
    Mul(Arc<Elementary>, Arc<Elementary>), // of the type f(x) * g(x)
    Div(Arc<Elementary>, Arc<Elementary>), // of the type f(x) / g(x)
    Pow(Arc<Elementary>, Arc<Elementary>), // of the type f(x)^g(x)
    Log(Arc<Elementary>, Arc<Elementary>), // of the type logb(f(x)) where b = g(x)

    // Absolute value function
    Abs(Arc<Elementary>),
    // Constant function
    Con(f64), // of the type c

    X, // unit function f(x) = x. Any function dependant on a variable must include this
       // function as it returns a function of type Func which returns the input value.
       // X will represent the independant variable in each function
}
impl Elementary {
    pub fn call(self) -> Func {
        Box::new(move |x| match self.clone() {
            Sin(func) => (*func).clone().call()(x).sin(),
            Cos(func) => (*func).clone().call()(x).cos(),
            Tan(func) => (*func).clone().call()(x).tan(),

            Asin(func) => (*func).clone().call()(x).asin(),
            Acos(func) => (*func).clone().call()(x).acos(),
            Atan(func) => (*func).clone().call()(x).atan(),

            Sinh(func) => {
                (E.powf((*func).clone().call()(x)) - E.powf(-(*func).clone().call()(x))) / 2.
            }
            Cosh(func) => {
                (E.powf((*func).clone().call()(x)) + E.powf(-(*func).clone().call()(x))) / 2.
            }
            Tanh(func) => Sinh(func.clone()).call()(x) / Cosh(func).call()(x),

            Add(func1, func2) => (*func1).clone().call()(x) + (*func2).clone().call()(x),
            Sub(func1, func2) => (*func1).clone().call()(x) - (*func2).clone().call()(x),
            Mul(func1, func2) => (*func1).clone().call()(x) * (*func2).clone().call()(x),
            Div(func1, func2) => (*func1).clone().call()(x) / (*func2).clone().call()(x),

            Pow(func1, func2) => (*func1).clone().call()(x).powf((*func2).clone().call()(x)),
            Log(func1, func2) => (*func2).clone().call()(x).log((*func1).clone().call()(x)),

            Abs(func) => (*func).clone().call()(x).abs(),

            Con(numb) => numb,

            X => f()(x),
        })
    }
}
// operation implementations for Elementary enum
impl Add for Elementary {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Add(Arc::new(self), Arc::new(rhs))
    }
}
impl Sub for Elementary {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Sub(Arc::new(self), Arc::new(rhs))
    }
}
impl Mul for Elementary {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Mul(Arc::new(self), Arc::new(rhs))
    }
}
impl Div for Elementary {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Div(Arc::new(self), Arc::new(rhs))
    }
}

// operation implementations for Elementary enum (with constants)
impl Add<f64> for Elementary {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self::Add(Arc::new(self), Arc::new(Con(rhs)))
    }
}

// basic trig functions
pub fn sin(func: Elementary) -> Elementary {
    Sin(Arc::new(func))
}
pub fn cos(func: Elementary) -> Elementary {
    Cos(Arc::new(func))
}
pub fn tan(func: Elementary) -> Elementary {
    Tan(Arc::new(func))
}

// basic arcus functions
pub fn asin(func: Elementary) -> Elementary {
    Asin(Arc::new(func))
}
pub fn acos(func: Elementary) -> Elementary {
    Acos(Arc::new(func))
}
pub fn atan(func: Elementary) -> Elementary {
    Atan(Arc::new(func))
}

// hyperbolic functions
pub fn sinh(func: Elementary) -> Elementary {
    Sinh(Arc::new(func))
}
pub fn cosh(func: Elementary) -> Elementary {
    Cosh(Arc::new(func))
}
pub fn tanh(func: Elementary) -> Elementary {
    Tanh(Arc::new(func))
}
// abs function
pub fn abs(func: Elementary) -> Elementary {
    Abs(Arc::new(func))
}

pub struct Function {
    pub func: Elementary,
}

impl Function {
    pub fn new(func: Elementary) -> Self {
        Self { func }
    }
}
impl<'a> From<&'a String> for Function {
    fn from(value: &'a String) -> Self {
        let func = Elementary::from(value);
        Self { func }
    }
}
