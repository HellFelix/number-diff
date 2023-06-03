use crate::Func;

/// returns n! for numbers n ∈ ℕ
pub fn factorial(numb: u8) -> u128 {
    if numb == 0 {
        1
    } else {
        let mut res: u128 = 1;
        for i in 1..=numb {
            res *= i as u128;
        }

        res
    }
}

pub fn maximum(f: Func, x1: f64, x2: f64) -> f64 {
    let potential_values = vec![f(x1), f(x2)];
    potential_values
        .iter()
        .fold(std::f64::MIN, |a, b| a.max(*b))
}

pub fn golden_section_search(f: Func, mut x1: f64, mut x2: f64) -> (f64, f64) {
    let tol = 1e-5;
    let inv_phi = (5_f64.sqrt() - 1.) / 2.;
    let inv_phi2 = (3. - 5_f64.sqrt()) / 2.;

    (x1, x2) = (x1.max(x2), x1.min(x2));

    let mut h = x2 - x1;

    if h <= tol {
        return (x1, x2);
    }

    let n = ((tol / h).log10() / inv_phi.log10()).ceil() as u32;

    let mut c = x1 + inv_phi2 * h;
    let mut d = x1 + inv_phi * h;
    let mut fc = f(c);
    let mut fd = f(d);

    for _ in 0..=n - 1 {
        if fc < fd {
            x2 = d;
            d = c;
            fd = fc;
            h *= inv_phi;
            c = x1 + inv_phi2 * h;
            fc = f(c);
        } else {
            x1 = c;
            c = d;
            fc = fd;
            h *= inv_phi;
            d = x1 + inv_phi * h;
            fd = f(d);
        }
    }

    if fc < fd {
        (x1, d)
    } else {
        (c, x2)
    }
}
