use crate::{tileboard::Coord, traits::Distance};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Manhattan;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Euclidian;

impl Distance for Manhattan {
    type Point = Coord;

    fn distance(a: Self::Point, b: Self::Point) -> f64 {
        let col_diff = (a.column() as isize - b.column() as isize).abs() as f64;
        let row_diff = (a.row() as isize - b.row() as isize).abs() as f64;
        (col_diff + row_diff) as f64
    }
}

impl Distance for Euclidian {
    type Point = Coord;

    fn distance(a: Self::Point, b: Self::Point) -> f64 {
        let col_diff = (a.column() as isize - b.column() as isize).abs() as f64;
        let row_diff = (a.row() as isize - b.row() as isize).abs() as f64;
        (col_diff.powi(2) + row_diff.powi(2)).sqrt() / 2f64
    }
}
