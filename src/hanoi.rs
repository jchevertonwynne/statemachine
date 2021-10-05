use std::fmt::Debug;

use arrayvec::ArrayVec;

use crate::{Coord, traits::State};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Hanoi<const RINGS: usize> {
    left: ArrayVec<usize, RINGS>,
    middle: ArrayVec<usize, RINGS>,
    right: ArrayVec<usize, RINGS>
}

impl<const RINGS: usize> Debug for Hanoi<RINGS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hanoi {{
            left: {:?},
            middle: {:?}.
            right: {:?}
        }}\n", self.left, self.middle, self.right)
    }
}

impl<const RINGS: usize> Default for Hanoi<RINGS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const RINGS: usize> Hanoi<RINGS> {
    pub fn new() -> Self {
        let mut res = Self {
            left: ArrayVec::new(),
            middle: ArrayVec::new(),
            right: ArrayVec::new()
        };

        for i in (1..=RINGS).rev() {
            res.left.push(i);
        }

        res
    }

    pub fn solved() -> Self {
        let mut res = Self {
            left: ArrayVec::new(),
            middle: ArrayVec::new(),
            right: ArrayVec::new()
        };

        for i in (1..=RINGS).rev() {
            res.right.push(i);
        }

        res
    }
}

impl<const RINGS: usize> State for Hanoi<RINGS> {
    type Point = Coord;

    fn next(&self) -> Vec<Self> {
        let mut res = Vec::new();

        let mut copy = self.clone();

        if let Some(tile) = copy.left.pop() {
            let middle = self.middle.last();
            if middle.is_none() || tile < *middle.unwrap() {
                let mut copy = self.clone();
                copy.middle.push(copy.left.pop().unwrap());
                res.push(copy);
            }

            let right = self.right.last();
            if right.is_none() || tile < *right.unwrap() {
                let mut copy = self.clone();
                copy.right.push(copy.left.pop().unwrap());
                res.push(copy);
            }
        }

        if let Some(tile) = copy.middle.pop() {
            let left = self.left.last();
            if left.is_none() || tile < *left.unwrap() {
                let mut copy = self.clone();
                copy.left.push(copy.middle.pop().unwrap());
                res.push(copy);
            }

            let right = self.right.last();
            if right.is_none() || tile < *right.unwrap() {
                let mut copy = self.clone();
                copy.right.push(copy.middle.pop().unwrap());
                res.push(copy);
            }
        }

        if let Some(tile) = copy.right.pop() {
            let left = self.left.last();
            if left.is_none() || tile < *left.unwrap() {
                let mut copy = self.clone();
                copy.left.push(copy.right.pop().unwrap());
                res.push(copy);
            }

            let middle = self.middle.last();
            if middle.is_none() || tile < *middle.unwrap() {
                let mut copy = self.clone();
                copy.middle.push(copy.right.pop().unwrap());
                res.push(copy);
            }
        }

        res
    }

    fn differences(&self) -> Vec<(Self::Point, Self::Point)> {
        let mut placements = vec![Coord::new(0, 0); RINGS];

        for (ind, &ring) in self.left.iter().enumerate() {
            placements[ring - 1] = Coord::new(0, ind);
        }

        for (ind, &ring) in self.middle.iter().enumerate() {
            placements[ring - 1] = Coord::new(1, ind);
        }

        for (ind, &ring) in self.right.iter().enumerate() {
            placements[ring - 1] = Coord::new(2, ind);
        }

        placements.reverse();

        let mut res = Vec::with_capacity(RINGS);

        for (intended_ind, ring) in placements.into_iter().enumerate() {
            res.push((Coord::new(3, intended_ind), ring));
        }

        res
    }
}