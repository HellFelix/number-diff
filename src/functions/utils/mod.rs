pub mod useful_functions;
pub use useful_functions::*;

#[cfg(feature = "nightly")]
pub mod calc_nightly;

#[cfg(feature = "default")]
pub mod calc_default;
