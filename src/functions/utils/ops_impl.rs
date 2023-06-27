use super::super::calc::force_unwrap;
use crate::Elementary::{self, *};
use crate::Function;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::sync::Arc;

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

// operation implementations for Function struct

impl Add for Function {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.elementary() + rhs.elementary())
    }
}
impl Add<&mut Self> for Function {
    type Output = Self;
    fn add(self, rhs: &mut Self) -> Self::Output {
        Self::from(self.elementary() + rhs.elementary())
    }
}
impl AddAssign for Function {
    fn add_assign(&mut self, rhs: Self) {
        self.set_function(self.elementary() + rhs.elementary());
    }
}

impl Sub for Function {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.elementary() - rhs.elementary())
    }
}
impl SubAssign for Function {
    fn sub_assign(&mut self, rhs: Self) {
        self.set_function(self.elementary() - rhs.elementary())
    }
}

impl Mul for Function {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.elementary() * rhs.elementary())
    }
}
impl Mul<&mut Self> for Function {
    type Output = Self;
    fn mul(self, rhs: &mut Self) -> Self::Output {
        Self::from(self.elementary() * rhs.elementary())
    }
}
impl MulAssign for Function {
    fn mul_assign(&mut self, rhs: Self) {
        self.set_function(self.elementary() * rhs.elementary())
    }
}
impl Div for Function {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::from(self.elementary() / rhs.elementary())
    }
}
impl Div<&mut Self> for Function {
    type Output = Self;
    fn div(self, rhs: &mut Self) -> Self::Output {
        Self::from(self.elementary() / rhs.elementary())
    }
}
impl DivAssign for Function {
    fn div_assign(&mut self, rhs: Self) {
        self.set_function(self.elementary() / rhs.elementary())
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
