use crate::Func;

pub fn integrate(func: Func, start: f64, stop: f64, dx: f64) -> f64 {
    simpsons_method(func, start, stop, ((stop-start)/dx) as u64)
}

fn trapezoid_method(func: Func, start: f64, stop: f64, dx: f64) -> f64 {
    let mut tot_area = 0.;
    let mut points: Vec<f64> = Vec::new();
    let mut x_value = start;
    while x_value < stop-dx {
        points.push(func(x_value));
        x_value += dx;
    }

    for i in 0..points.len() {
        if i != points.len()-1 {
            tot_area += trapeziod_area(points[i], points[i+1], dx);
        } else {
            tot_area += trapeziod_area(points[i], func(stop), stop - x_value);
        }
    }


    tot_area
}
fn trapeziod_area(a: f64, b: f64, h: f64) -> f64 {
    (a+b)*h/2.
}

fn simpsons_method(func: Func, start: f64, stop: f64, n: u64) -> f64 {
    let mut area = 0.;

    for k in 1..=n-1 {
        area += func(start + (k as f64)*((stop-start)/(n as f64)))
    }

    area += func(start/2.) + func(stop/2.);

    area *= (stop-start)/n as f64;

    area
}
