use crate::{
    Elementary::{self, *},
    Error,
};

#[derive(Debug, PartialEq)]
pub enum Category {
    Exponential,
    Polynomial,
    Trigonometric,
    Constant,
    ClusterFuck,
}

impl Elementary {
    pub fn classify(&self) -> Result<Category, Error> {
        if self.is_constant() {
            Ok(Category::Constant)
        } else if self.is_exponential()? {
            Ok(Category::Exponential)
        } else if self.is_polynomial()? {
            Ok(Category::Polynomial)
        } else if self.is_trig() {
            Ok(Category::Trigonometric)
        } else {
            Ok(Category::ClusterFuck)
        }
    }

    // checks if the provided function is constant, that is, it contains no independent variable
    // of the type f(x) = C
    pub fn is_constant(&self) -> bool {
        match self {
            Sin(func) => func.is_constant(),
            Cos(func) => func.is_constant(),
            Tan(func) => func.is_constant(),

            Sec(func) => func.is_constant(),
            Csc(func) => func.is_constant(),
            Cot(func) => func.is_constant(),

            Asin(func) => func.is_constant(),
            Acos(func) => func.is_constant(),
            Atan(func) => func.is_constant(),

            Sinh(func) => func.is_constant(),
            Cosh(func) => func.is_constant(),
            Tanh(func) => func.is_constant(),

            // for the operations, both of their functions must be constant for the whole function
            // to be considered constant
            Add(func1, func2) => func1.is_constant() && func2.is_constant(),
            Sub(func1, func2) => func1.is_constant() && func2.is_constant(),
            Mul(func1, func2) => func1.is_constant() && func2.is_constant(),
            Div(func1, func2) => func1.is_constant() && func2.is_constant(),
            Pow(func1, func2) => func1.is_constant() && func2.is_constant(),
            Log(func1, func2) => func1.is_constant() && func2.is_constant(),

            Abs(func) => func.is_constant(),

            Con(_) => true,
            X => false,
        }
    }

    // returns true if the function is of type f(x) = Cx
    fn is_linear(&self) -> bool {
        if let Mul(func1, func2) = self {
            if (func1.is_constant() && (func2.is_linear() || func2.clone() == X.into()))
                || (func2.is_constant() && (func1.is_linear() || func1.clone() == X.into()))
            {
                return true;
            }
        }
        false
    }

    // returns true if the function is a constant digit f(x) = C, C ∈ ℤ
    pub fn is_digit(&self) -> Result<bool, Error> {
        if let Con(numb) = self {
            if numb.fract() == 0.0 {
                return Ok(true);
            }
        }
        Ok(false)
    }

    // returns true if the function is of type f(x) = a^(cx)
    pub fn is_exponential(&self) -> Result<bool, Error> {
        if let Pow(base, exp) = self {
            if base.is_constant() && exp.is_linear() {
                return Ok(true);
            }
        }
        if let Mul(func1, func2) = self {
            if (func1.is_exponential()? && func2.is_constant())
                || (func1.is_constant() && func2.is_exponential()?)
            {
                return Ok(true);
            }
        }

        Ok(false)
    }

    // returns true if the function is of type f(x) = ax^n + bx^(n-1) + cx^(n-2) + ...
    // (a, b, c, ... ∈ ℝ)
    fn is_polynomial(&self) -> Result<bool, Error> {
        if let Pow(base, exp) = self {
            if base.clone().is_polynomial()? && exp.is_digit()? {
                return Ok(true);
            }
        } else if self.clone() == X.into() {
            return Ok(true);
        } else if let Con(_) = self {
            return Ok(true);
        } else if let Mul(func1, func2) = self {
            if func1.is_polynomial()? && func2.is_polynomial()? {
                return Ok(true);
            }
        } else if let Add(func1, func2) = self {
            if func1.is_polynomial()? && func2.is_polynomial()? {
                return Ok(true);
            }
        } else if let Sub(func1, func2) = self {
            if func1.is_polynomial()? && func2.is_polynomial()? {
                return Ok(true);
            }
        } else if let Div(func1, func2) = self {
            if func1.is_polynomial()? && func2.is_polynomial()? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    // returns true if the function is of type trig * trig, constant * trig, trig^constant, or
    // log constant (trig)
    fn is_trig(&self) -> bool {
        match self {
            Sin(_) => true,
            Cos(_) => true,
            Tan(_) => true,

            Sec(_) => true,
            Csc(_) => true,
            Cot(_) => true,

            Asin(_) => true,
            Acos(_) => true,
            Atan(_) => true,

            Sinh(_) => true,
            Cosh(_) => true,
            Tanh(_) => true,

            Add(func1, func2) => func1.is_trig() && func2.is_trig(),
            Sub(func1, func2) => func1.is_trig() && func2.is_trig(),
            Mul(func1, func2) => {
                (func1.is_trig() && func2.is_trig()) // trig * trig
                    || (func1.is_trig() && func2.is_constant()) // trig * constant
                    || (func2.is_trig() && func1.is_constant()) // constant * trig
            }
            Div(func1, func2) => {
                (func1.is_trig() && func2.is_trig())
                    || (func1.is_trig() && func2.is_constant())
                    || (func2.is_trig() && func1.is_constant())
            }
            Pow(func1, func2) => func1.is_trig() && func2.is_constant(), // trig^constant
            Log(func1, func2) => func1.is_constant() && func2.is_trig(), // log constant (trig)

            Abs(_) => false,
            Con(_) => false,
            X => false,
        }
    }
}
