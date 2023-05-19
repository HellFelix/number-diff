//! # Overview
//! ## Number Diff - An all-purpose tool for calculus
//!
//!
//!
//! ### Functions
//! Number Diff is built around a calculus-like function, that is, a function that takes an
//! f64 as an argument, returning an f64 according to some specific rule. In the current state,
//! functions are limited to ƒ: ℝ ⟶ ℝ,  
//! There are plans to expand to ƒ: ℂ ⟶ ℂ in the not so distant future.
//!
//! #### Usage
//! Functions are represented by the Function struct. The Function struct can be created by either
//! parsing a string or building an Elementary enum and passing it to the Function. A Function
//! instance can then be used with the call(x) method.
//!
//! ```rust
//! // creating the function by parsing a string, in this case "4sin(x)"
//! let func1 = Function::from("4sin(x)");
//! assert_eq!(func1.call(PI/2.), 4.);
//!
//! // creating the function by passing an Elementary enum, in this case cos(x)*|sin(x)|
//! let func_enum = cos(X)*abs(sin(X));
//! let func2 = Function::new(func_enum);
//! assert_eq!(func2.call(-PI/4.), 0.5);
//! ```
//!
//! Using the nightly feature, the function instance itself is also callable as such:
//! ```rust
//! let func = Function::from("x^ln(x)");
//! assert_eq!(func(E), E);
//! ```
//!
//!
//!
//! ### Derivatives
//! ### Integrals

#![cfg_attr(feature = "nightly", feature(unboxed_closures))]
#![cfg_attr(feature = "nightly", feature(fn_traits))]
#![cfg_attr(feature = "nightly", feature(tuple_trait))]

mod functions;
use functions::calc::Elementary;

mod utils;
pub use utils::include::*;

mod simplify;

type Func = Box<dyn Fn(f64) -> f64 + 'static>;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    SimplifyError(Elementary),
    InternalError(String),
}
