use crate::{Elementary, Error};

use super::classification::Category;

// NOT DONE YET!!
pub fn simplify_polynomial(polynomial: &Elementary) -> Result<Elementary, Error> {
    polynomial.simplify_operations()
}

fn polynomial_long_division(
    polynomial1: Elementary,
    polynomial2: Elementary,
) -> Result<Elementary, Error> {
    if polynomial1.classify()? != Category::Polynomial
        || polynomial2.classify()? != Category::Polynomial
    {
        return Err(Error::InternalError(String::from(format!("Attempted to perform polynomial long division on non polynomial functions. Functions were of type {:?} and {:?}", polynomial1.classify(), polynomial2.classify()))));
    }
    unimplemented!()
}
