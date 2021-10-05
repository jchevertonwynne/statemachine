use std::fmt::Debug;

use arrayvec::ArrayVec;
use rand::prelude::SliceRandom;

use crate::traits::State;

#[derive(PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TileBoard<const C: usize, const R: usize> {
    inner: [[usize; C]; R],
}

impl<const C: usize, const R: usize> Default for TileBoard<C, R> {
    fn default() -> Self {
        let mut inner = [[0; C]; R];

        inner
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .zip(1..)
            .for_each(|(item, val)| *item = val);
        inner[R - 1][C - 1] = 0;

        Self { inner }
    }
}

impl<const C: usize, const R: usize> TileBoard<C, R> {
    pub fn shuffled(shuffles: usize) -> Self {
        let mut res: TileBoard<C, R> = TileBoard::default();

        let (mut pos_x, mut pos_y) = (C - 1, R - 1);

        let mut r = rand::thread_rng();
        let mut move_options: ArrayVec<(usize, usize), 4> = ArrayVec::new();
        (0..shuffles).for_each(|_| {
            move_options.clear();

            if pos_x > 0 {
                move_options.push((pos_x - 1, pos_y));
            }

            if pos_x < C - 1 {
                move_options.push((pos_x + 1, pos_y));
            }

            if pos_y > 0 {
                move_options.push((pos_x, pos_y - 1));
            }

            if pos_y < R - 1 {
                move_options.push((pos_x, pos_y + 1));
            }

            let (c_x, c_y) = *move_options.choose(&mut r).unwrap();
            let t = res.inner[c_y][c_x];
            res.inner[c_y][c_x] = res.inner[pos_y][pos_x];
            res.inner[pos_y][pos_x] = t;
            pos_x = c_x;
            pos_y = c_y;
        });

        res
    }
}

impl<const C: usize, const R: usize> State for TileBoard<C, R> {
    type Point = Coord;

    fn next(&self) -> Vec<Self> {
        let mut res = Vec::new();

        let mut x = 0;
        let mut y = 0;

        for i in 0..C {
            for j in 0..R {
                if self.inner[j][i] == 0 {
                    x = i;
                    y = j;
                }
            }
        }

        if x > 0 {
            let mut other = self.clone();
            other.inner[y][x] = self.inner[y][x - 1];
            other.inner[y][x - 1] = self.inner[y][x];
            res.push(other);
        }

        if x < C - 1 {
            let mut other = self.clone();
            other.inner[y][x] = self.inner[y][x + 1];
            other.inner[y][x + 1] = self.inner[y][x];
            res.push(other);
        }

        if y > 0 {
            let mut other = self.clone();
            other.inner[y][x] = self.inner[y - 1][x];
            other.inner[y - 1][x] = self.inner[y][x];
            res.push(other);
        }

        if y < R - 1 {
            let mut other = self.clone();
            other.inner[y][x] = self.inner[y + 1][x];
            other.inner[y + 1][x] = self.inner[y][x];
            res.push(other);
        }

        res
    }

    fn differences(&self) -> Vec<(Self::Point, Self::Point)> {
        let mut pos = vec![0; C * R];
        for i in 0..(C * R) {
            let val = self.inner[i / C][i % C];
            pos[val] = i;
        }

        let mut res = Vec::with_capacity(C * R);

        for (actual, found) in pos.iter().enumerate() {
            let real = if actual == 0 {
                Coord::new(C - 1, R - 1)
            } else {
                Coord::new((actual - 1) % C, (actual - 1) / C)
            };
            let found = Coord::new(found % C, found / C);
            res.push((real, found));
        }

        res
    }
}

impl<const C: usize, const R: usize> Debug for TileBoard<C, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.inner {
            write!(f, "\n{:?}", row)?;
        }

        std::fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn calculates_square_grid_differences_alright() {
        let t: TileBoard<2, 2> = TileBoard {
            inner: [[1, 2], [3, 0]],
        };
        assert_eq!(
            t.differences(),
            vec![
                (Coord::new(1, 1), Coord::new(1, 1)),
                (Coord::new(0, 0), Coord::new(0, 0)),
                (Coord::new(1, 0), Coord::new(1, 0)),
                (Coord::new(0, 1), Coord::new(0, 1)),
            ]
        );

        let t: TileBoard<2, 2> = TileBoard {
            inner: [[1, 3], [2, 0]],
        };
        assert_eq!(
            t.differences(),
            vec![
                (Coord::new(1, 1), Coord::new(1, 1)),
                (Coord::new(0, 0), Coord::new(0, 0)),
                (Coord::new(1, 0), Coord::new(0, 1)),
                (Coord::new(0, 1), Coord::new(1, 0)),
            ]
        );

        let t: TileBoard<4, 4> = TileBoard {
            inner: [[0, 5, 2, 10], [1, 8, 6, 3], [14, 4, 9, 12], [11, 7, 15, 13]],
        };
        assert_eq!(
            t.differences(),
            vec![
                (Coord::new(3, 3), Coord::new(0, 0)),
                (Coord::new(0, 0), Coord::new(0, 1)),
                (Coord::new(1, 0), Coord::new(2, 0)),
                (Coord::new(2, 0), Coord::new(3, 1)),
                (Coord::new(3, 0), Coord::new(1, 2)),
                (Coord::new(0, 1), Coord::new(1, 0)),
                (Coord::new(1, 1), Coord::new(2, 1)),
                (Coord::new(2, 1), Coord::new(1, 3)),
                (Coord::new(3, 1), Coord::new(1, 1)),
                (Coord::new(0, 2), Coord::new(2, 2)),
                (Coord::new(1, 2), Coord::new(3, 0)),
                (Coord::new(2, 2), Coord::new(0, 3)),
                (Coord::new(3, 2), Coord::new(3, 2)),
                (Coord::new(0, 3), Coord::new(3, 3)),
                (Coord::new(1, 3), Coord::new(0, 2)),
                (Coord::new(2, 3), Coord::new(2, 3)),
            ]
        );
    }

    #[test]
    fn calculates_rectangular_grid_differences_alright() {
        let t: TileBoard<4, 1> = TileBoard {
            inner: [[0, 1, 2, 3]],
        };
        assert_eq!(
            t.differences(),
            vec![
                (Coord::new(3, 0), Coord::new(0, 0)),
                (Coord::new(0, 0), Coord::new(1, 0)),
                (Coord::new(1, 0), Coord::new(2, 0)),
                (Coord::new(2, 0), Coord::new(3, 0)),
            ]
        );

        let t: TileBoard<4, 1> = TileBoard {
            inner: [[3, 2, 1, 0]],
        };
        assert_eq!(
            t.differences(),
            vec![
                (Coord::new(3, 0), Coord::new(3, 0)),
                (Coord::new(0, 0), Coord::new(2, 0)),
                (Coord::new(1, 0), Coord::new(1, 0)),
                (Coord::new(2, 0), Coord::new(0, 0)),
            ]
        );

        let t: TileBoard<4, 2> = TileBoard {
            inner: [[7, 6, 5, 4], [3, 2, 1, 0]],
        };
        assert_eq!(
            t.differences(),
            vec![
                (Coord::new(3, 1), Coord::new(3, 1)),
                (Coord::new(0, 0), Coord::new(2, 1)),
                (Coord::new(1, 0), Coord::new(1, 1)),
                (Coord::new(2, 0), Coord::new(0, 1)),
                (Coord::new(3, 0), Coord::new(3, 0)),
                (Coord::new(0, 1), Coord::new(2, 0)),
                (Coord::new(1, 1), Coord::new(1, 0)),
                (Coord::new(2, 1), Coord::new(0, 0)),
            ]
        );
    }
}
