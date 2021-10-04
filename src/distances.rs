use crate::traits::Distance;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Manhattan;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Euclidian;

impl Distance for Manhattan {
    fn distance(a: (usize, usize), b: (usize, usize)) -> f64 {
        let x_diff = (a.0 as isize - b.0 as isize).abs() as usize;
        let y_diff = (a.1 as isize - b.1 as isize).abs() as usize;
        (x_diff + y_diff) as f64
    }
}

impl Distance for Euclidian {
    fn distance(a: (usize, usize), b: (usize, usize)) -> f64 {
        let x_diff = (a.0 as isize - b.0 as isize) as f64;
        let y_diff = (a.1 as isize - b.1 as isize) as f64;
        (x_diff.powi(2) + y_diff.powi(2)).sqrt()
    }
}
