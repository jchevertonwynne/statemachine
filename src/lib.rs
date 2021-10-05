use std::fmt::Debug;

pub mod boxes;
pub mod distances;
pub mod machine;
pub mod sharedlist;
pub mod tileboard;
pub mod traits;
pub mod hanoi;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Coord {
    column: usize,
    row: usize,
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coord({}, {})", self.column, self.row)
    }
}

impl Coord {
    fn new(column: usize, row: usize) -> Self {
        Self { column, row }
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn row(&self) -> usize {
        self.row
    }
}