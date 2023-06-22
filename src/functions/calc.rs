use crate::{gamma_function, polygamma_function, Elementary::*, Factorial};
use std::{
    f64::consts::E,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub},
    sync::Arc,
};

use crate::{Error, Func};

use super::series_expansions::SeriesExpansion;

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

    Sec(Arc<Elementary>), // of the tyoe sec(f(x))
    Csc(Arc<Elementary>), // of the type csc(f(x))
    Cot(Arc<Elementary>), // of the type cot(f(x))

    // Standard arcus functions
    Asin(Arc<Elementary>), // of the type arcsin(f(x))
    Acos(Arc<Elementary>), // of the type arccos(f(x))
    Atan(Arc<Elementary>), // of the type arctan(f(x))

    // hyperbolic trig functions
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

    // special functions
    Factorial(Arc<Elementary>),
    // gamma function
    Gamma(Arc<Elementary>),            // of the type 𝜞(f(x))
    Polygamma(Arc<Elementary>, usize), // Of the type 𝝍m(f(x))

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

            Sec(func) => 1. / (*func).clone().call()(x).cos(),
            Csc(func) => 1. / (*func).clone().call()(x).sin(),
            Cot(func) => 1. / (*func).clone().call()(x).tan(),

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

            Factorial(func) => (*func).clone().call()(x).factorial(),

            Gamma(func) => gamma_function((*func).clone().call()(x)),
            Polygamma(func, order) => polygamma_function((*func).clone().call()(x), order),

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
        if let Con(numb) = self {
            if numb == 0. {
                return rhs;
            }
        } else if let Con(numb) = rhs {
            if numb == 0. {
                return self;
            }
        }
        Self::Add(Arc::new(self), Arc::new(rhs))
    }
}
impl Add<&mut Self> for Elementary {
    type Output = Self;
    fn add(self, rhs: &mut Self) -> Self::Output {
        self + rhs.clone()
    }
}
impl AddAssign for Elementary {
    fn add_assign(&mut self, rhs: Self) {
        *self = rhs + self.clone();
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
        if let Con(numb) = self {
            if numb == 1. {
                return rhs;
            } else if numb == 0. {
                return self;
            }
        } else if let Con(numb) = rhs {
            if numb == 1. {
                return self;
            } else if numb == 0. {
                return rhs;
            }
        }
        Self::Mul(Arc::new(self), Arc::new(rhs))
    }
}
impl Mul<&mut Self> for Elementary {
    type Output = Self;
    fn mul(self, rhs: &mut Self) -> Self::Output {
        self * rhs.clone()
    }
}
impl MulAssign for Elementary {
    fn mul_assign(&mut self, rhs: Self) {
        *self = rhs * self.clone();
    }
}
impl Div for Elementary {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Div(Arc::new(self), Arc::new(rhs))
    }
}
impl Div<&mut Self> for Elementary {
    type Output = Self;
    fn div(self, rhs: &mut Self) -> Self::Output {
        self / rhs.clone()
    }
}
impl DivAssign for Elementary {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

// operation implementations for Elementary enum (with constants)
impl Add<f64> for Elementary {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self::Add(Arc::new(self), Arc::new(Con(rhs)))
    }
}
impl Sub<f64> for Elementary {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self::Sub(Arc::new(self), Arc::new(Con(rhs)))
    }
}
impl Div<f64> for Elementary {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Div(Arc::new(self), Arc::new(Con(rhs)))
    }
}
impl Mul<f64> for Elementary {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Mul(Arc::new(self), Arc::new(Con(rhs)))
    }
}

// operation implementations for Arc<Elementary>
impl Add<Elementary> for Arc<Elementary> {
    type Output = Elementary;
    fn add(self, rhs: Elementary) -> Self::Output {
        let elem = force_unwrap(&self);
        elem + rhs
    }
}
impl Sub<Elementary> for Arc<Elementary> {
    type Output = Elementary;
    fn sub(self, rhs: Elementary) -> Self::Output {
        let elem = force_unwrap(&self);
        elem - rhs
    }
}
impl Mul<Elementary> for Arc<Elementary> {
    type Output = Elementary;
    fn mul(self, rhs: Elementary) -> Self::Output {
        let elem = force_unwrap(&self);
        elem * rhs
    }
}
impl Div<Elementary> for Arc<Elementary> {
    type Output = Elementary;
    fn div(self, rhs: Elementary) -> Self::Output {
        let elem = force_unwrap(&self);
        elem / rhs
    }
}

impl Sum<Self> for Elementary {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let terms: Vec<Elementary> = iter.collect();
        let mut res = terms[0].clone();
        for i in 1..terms.len() {
            res += terms[i].clone();
        }
        res
    }
}

// returns the inner Elementary value of the Arc or returns a clone
pub fn force_unwrap(element: &Arc<Elementary>) -> Elementary {
    if let Ok(inner) = Arc::try_unwrap(element.clone()) {
        inner
    } else {
        (**element).clone()
    }
}

// basic trig functions
/// Creates a [Function](crate::Function) equal to the sine of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ sin(f(x))
///
/// Example:
/// ```rust
///     let x = Function::default();
///     let sin_of_x = sin(x);
///     assert_eq!(sin_of_x.call(PI / 2.), 1.);
/// ```
pub fn sin(func: Function) -> Function {
    let new_function = Sin(Arc::new(func.elementary()));
    Function::from(new_function)
}

/// Creates a [Function](crate::Function) equal to the cosine of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ cos(f(x))
///
/// Example:
/// ```rust
///     let x = Function::default();
///     let cos_of_x = cos(x);
///     assert_eq!(cos_of_x.call(0.), 1.);
/// ```
pub fn cos(func: Function) -> Function {
    let new_function = Cos(Arc::new(func.elementary()));
    Function::from(new_function)
}

/// Creates a [Function](crate::Function) equal to the tangent of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ tan(f(x))
///
/// Example:
/// ```rust
///     let x = Function::default();
///     let tan_of_x = tan(x);
///     assert_eq!(tan_of_x.call(PI / 4.), 1.);
/// ```
pub fn tan(func: Function) -> Function {
    let new_function = Tan(Arc::new(func.elementary()));
    Function::from(new_function)
}

/// Creates a [Function](crate::Function) equal to the secant of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ sec(f(x))
///
/// Example:
/// ```rust
///     let x = Function::default();
///     let sec_of_x = sec(x);
///     assert_eq!(sec_of_x.call(PI), -1.);
/// ```
pub fn sec(func: Function) -> Function {
    let new_function = Sec(Arc::new(func.elementary()));
    Function::from(new_function)
}

/// Creates a [Function](crate::Function) equal to the cosecant of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ csc(f(x))
///
/// Example:
/// ```rust
///     let x = Function::default();
///     let csc_of_x = csc(x);
///     assert_eq!(csc_of_x.call(3./2. * PI), -1.);
/// ```
pub fn csc(func: Function) -> Function {
    let new_function = Csc(Arc::new(func.elementary()));
    Function::from(new_function)
}

/// Creates a [Function](crate::Function) equal to the cotangent of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ cot(f(x))
///
/// Example:
/// ```rust
///     let x = Function::default();
///     let cot_of_x = cot(x);
///     assert_eq!(cot_of_x.call(PI/2.), 0.);
/// ```
pub fn cot(func: Function) -> Function {
    let new_function = Cot(Arc::new(func.elementary()));
    Function::from(new_function)
}

// basic arcus functions
/// Creates a [Function](crate::Function) equal to the arcsine of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ asin(f(x))
///
/// Example:
/// ```rust
///     let x = function::default();
///     let asin_of_x = asin(x);
///     assert_eq!(asin_of_x.call(1.), PI/2.);
/// ```
pub fn asin(func: Function) -> Function {
    let new_function = Asin(Arc::new(func.elementary()));
    Function::from(new_function)
}

/// Creates a [Function](crate::Function) equal to the arccosine of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ acos(f(x))
///
/// Example:
/// ```rust
///     let x = function::default();
///     let acos_of_x = acos(x);
///     assert_eq!(acos_of_x.call(1.), 0.);
/// ```
pub fn acos(func: Function) -> Function {
    let new_function = Acos(Arc::new(func.elementary()));
    Function::from(new_function)
}

/// Creates a [Function](crate::Function) equal to the arctangent of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ atan(f(x))
///
/// Example:
/// ```rust
///     let x = function::default();
///     let atan_of_x = atan(x);
///     assert_eq!(atan_of_x.call(1.), PI/4.);
/// ```
pub fn atan(func: Function) -> Function {
    let new_function = Atan(Arc::new(func.elementary()));
    Function::from(new_function)
}

// hyperbolic functions

/// Creates a [Function](crate::Function) equal to the hyperbolic sine of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ sinh(f(x))
///
/// Example:
/// ```rust
///     let x = function::default();
///     let sinh_of_x = sinh(x);
///     assert_eq!(sinh_of_x.call(0.), 0.);
/// ```
pub fn sinh(func: Function) -> Function {
    let new_function = Sinh(Arc::new(func.elementary()));
    Function::from(new_function)
}

/// Creates a [Function](crate::Function) equal to the hyperbolic cosine of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ cosh(f(x))
///
/// Example:
/// ```rust
///     let x = function::default();
///     let cosh_of_x= cosh(x);
///     assert_eq!(cosh_of_x.call(0.), 1.);
/// ```
pub fn cosh(func: Function) -> Function {
    let new_function = Cosh(Arc::new(func.elementary()));
    Function::from(new_function)
}

/// Creates a [Function](crate::Function) equal to the hyperbolic tangent of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ tanh(f(x))
///
/// Example:
/// ```rust
///     let x = function::default();
///     let tanh_of_x= tanh(x);
///     assert_eq!(tanh_of_x.call(0.), 0.);
/// ```
pub fn tanh(func: Function) -> Function {
    let new_function = Tanh(Arc::new(func.elementary()));
    Function::from(new_function)
}
// abs function
/// Creates a [Function](crate::Function) equal to the absolute value of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ |f(x)|
///
/// Example:
/// ```rust
///     let x = function::default();
///     let abs_of_x= abs(x);
///     assert_eq!(abs_of_x.call(-1.), 1.);
/// ```
pub fn abs(func: Function) -> Function {
    let new_function = Abs(Arc::new(func.elementary()));
    Function::from(new_function)
}

/// Creates a [Function](crate::Function) equal to the square root of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ √f(x)
///
/// Example:
/// ```rust
///     let x = function::default();
///     let sqrt_of_x= sqrt(x);
///     assert_eq!(sqrt_of_x.call(4.), 2.);
/// ```
pub fn sqrt(func: Function) -> Function {
    let new_function = Pow(Arc::new(func.elementary()), Arc::new(Con(0.5)));
    Function::from(new_function)
}

/// Creates a [Function](crate::Function) equal to the nth root of the passed [Function](crate::Function)
///
/// i.e f(x) ⟹ √f(x)
///
/// Example:
/// ```rust
///     let x = function::default();
///     let nth_root_of_x= nth_root(x, 3);
///     assert_eq!(nth_root_of_x.call(8.), 2.);
/// ```
pub fn nth_root(func: Function, n: f64) -> Function {
    let new_function = Pow(Arc::new(func.elementary()), Arc::new(Con(1.) / n));
    Function::from(new_function)
}

pub struct Function {
    func: Elementary,
}

impl Function {
    pub fn elementary(&self) -> Elementary {
        self.func.clone()
    }

    /// Turns self into the derivative of self
    ///
    /// i.e. f(x) ⟹ f'(x)
    pub fn differentiate(&mut self) -> Result<(), Error> {
        self.func = self.elementary().to_owned().derivative()?;
        Ok(())
    }

    /// Turns the given [Function](crate::Function) instance into a Taylor series expansion centered around the value
    /// of a.
    ///
    /// If the conversion fails, an [Error::ExpansionError](Error) is returned.
    pub fn as_taylor_expansion(&mut self, order: u8, a: f64) -> Result<(), Error> {
        self.func = self.func.expand_taylor(order, a)?.get_elementary();
        Ok(())
    }

    ///
    pub fn get_taylor_expansion(&self, order: u8, a: f64) -> Result<SeriesExpansion, Error> {
        self.func.expand_taylor(order, a)
    }
}
impl Default for Function {
    /// The default() method returns the unit function f(x) = x, returning the independant variable
    /// for each input value.
    fn default() -> Self {
        Self { func: X }
    }
}

/// A [Function](crate::Function) instance can be parsed from any string type using the from method.
///
/// Example:
/// ```rust
/// let func = Function::from("sin(ln(x))");
///
/// assert_eq!(func.call(1.), 0.);
/// // ...or using the nightly feature
/// // assert_eq!(func(1.), 0.);
/// ```
impl<'a> From<&'a str> for Function {
    fn from(value: &'a str) -> Self {
        let func = Elementary::from(value);
        Self { func }
    }
}
impl From<String> for Function {
    fn from(value: String) -> Self {
        let func = Elementary::from(&value[..]);
        Self { func }
    }
}
impl<'a> From<&'a String> for Function {
    fn from(value: &'a String) -> Self {
        let func = Elementary::from(&value[..]);
        Self { func }
    }
}
impl From<Elementary> for Function {
    fn from(value: Elementary) -> Self {
        Self { func: value }
    }
}
/// A [Function](crate::Function) instance can be obtained from a SeriesExpansion instance using the from method.
///
/// Example:
/// ```rust
/// // create the Function instance
/// let func = Function::from("sin(x)");
///
/// // Get the SeriesExpansion
/// // In this instance we're creating a Taylor expansion of order 5 centered around 0
/// let expansion = func.get_taylor_expansion(5, 0.);
///
/// // Convert the SeriesExpansion into a Function using the from method
/// let func_expansion = Function::from(expansion);
/// // Note that this could also be done using the get_function method:
/// // let func_expansion = expansion.get_function()
/// //
/// // ... or using the as_taylor_expansion method to convert the original Function instance into a
/// // Taylor expansion without creating the SeriesExpansion instance seperatly:
/// // func.as_taylor_expansion()
///
/// ```
impl From<SeriesExpansion> for Function {
    fn from(value: SeriesExpansion) -> Self {
        value.get_function()
    }
}
impl From<&SeriesExpansion> for Function {
    fn from(value: &SeriesExpansion) -> Self {
        value.get_function()
    }
}
