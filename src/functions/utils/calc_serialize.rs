use crate::Elementary::{self, *};
use crate::Function;

impl serde::Serialize for Elementary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Sin(func) => serializer.serialize_newtype_variant("Elementary", 0, "Sin", &(**func)),
            Cos(func) => serializer.serialize_newtype_variant("Elementary", 1, "Cos", &(**func)),
            Tan(func) => serializer.serialize_newtype_variant("Elementary", 2, "Tan", &(**func)),

            Sec(func) => serializer.serialize_newtype_variant("Elementary", 3, "Sec", &(**func)),
            Csc(func) => serializer.serialize_newtype_variant("Elementary", 4, "Csc", &(**func)),
            Cot(func) => serializer.serialize_newtype_variant("Elementary", 5, "Cot", &(**func)),

            Asin(func) => serializer.serialize_newtype_variant("Elementary", 6, "Asin", &(**func)),
            Acos(func) => serializer.serialize_newtype_variant("Elementary", 7, "Acos", &(**func)),
            Atan(func) => serializer.serialize_newtype_variant("Elementary", 8, "Atan", &(**func)),

            Sinh(func) => serializer.serialize_newtype_variant("Elementary", 9, "Sinh", &(**func)),
            Cosh(func) => serializer.serialize_newtype_variant("Elementary", 10, "Cosh", &(**func)),
            Tanh(func) => serializer.serialize_newtype_variant("Elementary", 11, "Tanh", &(**func)),

            Add(func1, func2) => serializer.serialize_newtype_variant(
                "Elementary",
                12,
                "Add",
                &vec![((**func1).clone()), ((**func2).clone())],
            ),
            Sub(func1, func2) => serializer.serialize_newtype_variant(
                "Elementary",
                13,
                "Sub",
                &vec![((**func1).clone()), ((**func2).clone())],
            ),
            Mul(func1, func2) => serializer.serialize_newtype_variant(
                "Elementary",
                14,
                "Mul",
                &vec![((**func1).clone()), ((**func2).clone())],
            ),
            Div(func1, func2) => serializer.serialize_newtype_variant(
                "Elementary",
                15,
                "Div",
                &vec![((**func1).clone()), ((**func2).clone())],
            ),
            Pow(func1, func2) => serializer.serialize_newtype_variant(
                "Elementary",
                16,
                "Pow",
                &vec![((**func1).clone()), ((**func2).clone())],
            ),
            Log(func1, func2) => serializer.serialize_newtype_variant(
                "Elementary",
                17,
                "Log",
                &vec![((**func1).clone()), ((**func2).clone())],
            ),
            Factorial(func) => {
                serializer.serialize_newtype_variant("Elementary", 18, "Factorial", &(**func))
            }

            Gamma(func) => {
                serializer.serialize_newtype_variant("Elementary", 19, "Gamma", &(**func))
            }
            Polygamma(func, order) => serializer.serialize_newtype_variant(
                "Elementary",
                20,
                "PolyGamma",
                &((**func).clone(), *order),
            ),

            Abs(func) => serializer.serialize_newtype_variant("Elementary", 21, "Abs", &(**func)),

            Con(numb) => serializer.serialize_newtype_variant("Elementary", 22, "Con", &(*numb)),

            X => serializer.serialize_unit_variant("Elementary", 23, "X"),
        }
    }
}

/// Serialize function into an [Elementary](crate::Elementary) representation.
///
/// Note: Serialization requires the use of the `serialize` feature!
/// ```
/// number-diff = { version = "^0.1", features = ["serialize"] }
/// ```
///
/// Example (using serde_json):
/// ```rust
/// // initialize a function instance
/// let function = Function::from("sin(cos(x^x + 3))");
///
/// // serialize to json string
/// let json = serde_json::to_string(&function).unwrap();
///
/// // the outcome will be a Elementary representation of the function in json format
/// let expected_json = r#"{"Elementary":{"Sin":{"Cos":{"Add":[{"Con":3.0},{"Pow":["X","X"]}]}}}}"#;
///
/// assert_eq!(json, expected_json);
///
/// ```
impl serde::Serialize for Function {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("Person", 3)?;
        s.serialize_field("Elementary", &self.elementary())?;
        s.end()
    }
}
