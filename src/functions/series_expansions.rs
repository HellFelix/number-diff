use std::sync::Arc;

use crate::{
    Elementary::{self, *},
    Error, Factorial, Function,
};

#[derive(Debug, Clone)]
pub enum SeriesExpansion {
    MacLaurin(Elementary),
    Taylor(Elementary),
    // Fourier,
    // Dirichlet,
}
impl SeriesExpansion {
    pub fn get_function(&self) -> Function {
        match self {
            SeriesExpansion::MacLaurin(elem) => Function::from(elem.clone()),
            SeriesExpansion::Taylor(elem) => Function::from(elem.clone()),
        }
    }

    pub fn get_elementary(&self) -> Elementary {
        match self {
            SeriesExpansion::MacLaurin(elem) => elem.clone(),
            SeriesExpansion::Taylor(elem) => elem.clone(),
        }
    }
}

impl Elementary {
    pub fn expand_maclaurin(&self, order: u8) -> Result<SeriesExpansion, Error> {
        let series = self.expand_taylor(order, 0.)?;

        if let SeriesExpansion::Taylor(res) = series {
            Ok(SeriesExpansion::MacLaurin(res))
        } else {
            unreachable!()
        }
    }

    pub fn expand_taylor(&self, order: u8, centre: f64) -> Result<SeriesExpansion, Error> {
        let mut terms: Vec<Elementary> = Vec::new();

        let mut current_derivative = self.clone();

        let first_term = current_derivative.clone().call()(centre);

        terms.push(Con(first_term));

        for i in 1..=order {
            current_derivative = current_derivative.derivative_unsimplified();

            let ith_term = Pow(Arc::new(X - centre), Arc::new(Con(i as f64)))
                * current_derivative.clone().call()(centre)
                / ((i as usize).factorial() as f64);

            terms.push(ith_term);
        }

        let mut res = Con(0.);

        for term in terms {
            res += term;
        }

        res = res.simplify()?;

        // TODO: check the result against lagrange_error_bound to make sure the maximum error is
        // within the bound

        Ok(SeriesExpansion::Taylor(res))
    }
}
