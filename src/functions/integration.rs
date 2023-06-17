use crate::{
    Elementary::{self, *},
    Error,
};

use super::super::simplify::polynomial::convert_term;

// The way to handle integration:
//
// 1, Create a series expansion at both start and stop
// 2. Get primative functions of both series expansions (polynomials are easy to integrate)
// 3. Evaluate the primative function at their respective seriesexpansion centre
// 4. Evaluate the integral
impl Elementary {
    pub fn integrate(&self, start: f64, stop: f64, precision: u8) -> Result<f64, Error> {
        let lower_bound = self.get_primative_at(start, precision)?;
        let upper_bound = self.get_primative_at(stop, precision)?;
        println!("{lower_bound}");
        println!("{upper_bound}");
        Ok(upper_bound - lower_bound)
    }

    fn get_primative_at(&self, point: f64, precision: u8) -> Result<f64, Error> {
        let series = self.expand_taylor(precision, point)?.get_elementary();

        let anti_derivative = series.integrate_polynomial()?;

        Ok(anti_derivative.call()(point))
    }

    // coming into this function, the polynomials should be fully simplified such that each term is of
    // the form ax^b
    // These terms are then integrated seperatly.
    fn integrate_polynomial(&self) -> Result<Elementary, Error> {
        let terms = self.break_polynomial()?;

        let primative: Elementary = terms
            .iter()
            .map(|x| {
                if let Mul(coefficient, power) = x {
                    if let Pow(_, exp) = (**power).clone() {
                        Self::integrate_polynomial_chunk(
                            (**coefficient).clone().call()(0.),
                            (*exp).clone().call()(0.),
                        )
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            })
            .sum();

        Ok(primative)
    }

    fn break_polynomial(&self) -> Result<Vec<Elementary>, Error> {
        let mut chunks: Vec<Elementary> = Vec::new();

        self.iterate_break(&mut chunks)?;

        Ok(chunks)
    }

    fn iterate_break(&self, chunks: &mut Vec<Elementary>) -> Result<(), Error> {
        if let Add(chunk, new_polynomial) = self {
            chunks.push(convert_term((**chunk).clone())?);
            (*new_polynomial).clone().iterate_break(chunks)?;
        } else {
            // it's going to be a polynomial chunk
            chunks.push(convert_term((*self).clone())?);
        }
        Ok(())
    }

    fn integrate_polynomial_chunk(coefficient: f64, exponent: f64) -> Elementary {
        let new_exponent = exponent + 1.;
        let new_coefficient = coefficient / (new_exponent);

        let term = Mul(
            Con(new_coefficient).into(),
            Pow(X.into(), Con(new_exponent).into()).into(),
        );
        term
    }
}
