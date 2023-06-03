use std::f64::consts::PI;

use number_diff::{sin, Elementary::*, Function};

fn main() {
    let element = Add(
        Div(
            Mul(
                Pow(Sub(X.into(), Con(0.0).into()).into(), Con(3.0).into()).into(),
                Con(-1.0).into(),
            )
            .into(),
            Con(6.0).into(),
        )
        .into(),
        Add(
            Div(
                Mul(
                    Pow(Sub(X.into(), Con(0.0).into()).into(), Con(2.0).into()).into(),
                    Con(0.0).into(),
                )
                .into(),
                Con(2.0).into(),
            )
            .into(),
            Div(
                Mul(
                    Pow(Sub(X.into(), Con(0.0).into()).into(), Con(1.0).into()).into(),
                    Con(1.0).into(),
                )
                .into(),
                Con(1.0).into(),
            )
            .into(),
        )
        .into(),
    );

    let test_element = Mul(
        Pow(Sub(X.into(), Con(0.0).into()).into(), Con(3.0).into()).into(),
        Con(-1.0).into(),
    );
    println!("{:?}", test_element.simplify().unwrap());
}
