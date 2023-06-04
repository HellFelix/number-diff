use std::sync::Arc;

use crate::{
    Elementary::{self, *},
    Error,
};

use super::classification::Category;
use super::polynomial;

impl Elementary {
    pub fn simplify(&self) -> Result<Self, Error> {
        let new_function: Self = match self.classify()? {
            Category::Constant => self.simplify_constant()?,
            Category::Polynomial => polynomial::simplify_polynomial(self.clone())?,
            Category::ClusterFuck => self.simplify_operations()?,
            _ => self.clone(),
        };

        self.check_simplification(&new_function)
    }

    // makes sure that the simplified funciton is correct, that is, it will yield the same result
    // upon calling for all numbers within its definition set.
    fn check_simplification(&self, new_function: &Self) -> Result<Self, Error> {
        let callable_self = self.clone().call();
        let callable_new = new_function.clone().call();
        for i in -1000..1000 {
            if callable_self(i as f64) != callable_new(i as f64) {
                println!("{self:?}");
                println!("{new_function:?}");
                return Err(Error::InternalError(String::from(
                    format!("while attempting to simplify {self:?}, the simplification method yielded inconsistent results. Found that self({i}) != new_function({i}). Left hand side found to be {}. Right hand side found to be {}", callable_self(i as f64), callable_new(i as f64)))));
            }
        }
        Ok(new_function.to_owned())
    }

    // used for functions of category ClusterFuck in order to break down and simplify each
    // individual funciton individually.
    pub fn simplify_operations(&self) -> Result<Self, Error> {
        match self {
            Mul(func1, func2) => Ok(func1.simplify()? * func2.simplify()?),
            Div(func1, func2) => Ok((func1.simplify()? / func2.simplify()?).divide()?),
            Add(func1, func2) => Ok(func1.simplify()? + func2.simplify()?),
            Sub(func1, func2) => Ok(func1.simplify()? - func2.simplify()?),
            Pow(func1, func2) => Self::simplify_power(func1, func2),
            Log(func1, func2) => Ok(Log(
                Arc::new(func1.simplify()?),
                Arc::new(func2.simplify()?),
            )),
            _ => Ok(self.to_owned()),
        }
    }

    fn simplify_power(base: &Self, exp: &Self) -> Result<Self, Error> {
        match exp.clone() {
            Con(numb) => {
                if numb == 0. {
                    Ok(Con(1.))
                } else if numb == 1. {
                    Ok(base.clone())
                } else {
                    Ok(Pow(base.simplify()?.into(), exp.simplify()?.into()))
                }
            }
            _ => match base {
                X => Ok(Pow(base.clone().into(), exp.clone().into())),
                Pow(inner_base, inner_exp) => Ok(Pow(
                    inner_base.clone(),
                    (exp.clone() * (**inner_exp).clone()).into(),
                )),
                _ => Ok(Pow(base.simplify()?.into(), exp.simplify()?.into())),
            },
        }
    }

    pub fn divide(&self) -> Result<Self, Error> {
        if let Div(numerator, denomenator) = self {
            let numerator = numerator.factor()?;
            let denomenator = denomenator.factor()?;

            let mut removed_numerator: Vec<usize> = Vec::new();
            let mut removed_denomenator: Vec<usize> = Vec::new();

            let mut constant_factor = 1.;

            for i in 0..numerator.len() {
                for j in 0..denomenator.len() {
                    if numerator[i] == denomenator[j] {
                        removed_numerator.push(i);
                        removed_denomenator.push(j);
                    } else if let (Con(numb1), Con(numb2)) =
                        (numerator[i].clone(), denomenator[j].clone())
                    {
                        constant_factor *= numb1 / numb2;
                        removed_numerator.push(i);
                        removed_denomenator.push(j);
                    }
                }
            }

            let mut new_numerator = Con(constant_factor);
            for i in 0..numerator.len() {
                if !removed_numerator.contains(&i) {
                    new_numerator = new_numerator * numerator[i].clone();
                }
            }
            let mut new_denomenator = Con(1.);
            for i in 0..denomenator.len() {
                if !removed_denomenator.contains(&i) {
                    new_denomenator = new_denomenator * denomenator[i].clone();
                }
            }

            if new_denomenator == Con(1.) {
                Ok(new_numerator.simplify()?)
            } else {
                Ok(new_numerator / new_denomenator)
            }
        } else {
            Err(Error::SimplifyError(
                self.to_owned(),
                String::from("Attempted to divide a non-divisible expression while simplifying"),
            ))
        }
    }

    pub fn factor(&self) -> Result<Vec<Self>, Error> {
        let mut factors: Vec<Self> = Vec::new();
        if let Mul(func1, func2) = self {
            for factor in func1.factor()? {
                factors.push(factor);
            }
            for factor in func2.factor()? {
                factors.push(factor);
            }
        } else if let Add(func1, func2) = self {
            for f1 in func1.factor()? {
                for f2 in func2.factor()? {
                    if f1.clone() == f2.clone() {
                        factors.push(f1.clone());
                        factors.push(
                            (Div(func1.to_owned(), Arc::new(f1.clone())).divide()?
                                + Div(func2.to_owned(), Arc::new(f2.clone())).divide()?)
                            .simplify()?,
                        );
                    } else if let (Con(numb1), Con(numb2)) = (f1.clone(), f2.clone()) {
                        let gcd = Con(gcd(numb1, numb2));
                        factors.push(gcd.clone());
                        factors.push(
                            ((func1.clone() / gcd.clone()).divide()?)
                                + (func2.clone() / gcd).divide()?,
                        );
                    }
                }
            }
        } else {
            factors.push(self.to_owned());
        }

        let res: Vec<Self> = factors
            .iter()
            .filter(|e| **e != Con(1.))
            .map(|e| e.to_owned())
            .collect();

        Ok(res)
    }

    pub fn simplify_constant(&self) -> Result<Self, Error> {
        if self.classify()? == Category::Constant {
            let value = self.clone().call()(0.);
            Ok(Con(value))
        } else {
            Err(Error::SimplifyError(
                self.to_owned(),
                String::from("Attempted to constant-simplify a non-constant expression"),
            ))
        }
    }
}

fn gcd(numb1: f64, numb2: f64) -> f64 {
    // create bindings
    let mut numb1 = numb1;
    let mut numb2 = numb2;

    while numb2 != 0. {
        let temp = numb1;
        numb1 = numb2;
        numb2 = temp % numb2;
    }
    numb1
}
