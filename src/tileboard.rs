use std::{fmt::Debug, marker::PhantomData};

use arrayvec::ArrayVec;
use rand::prelude::SliceRandom;

use crate::traits::{Distance, State};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TileBoard<D: Distance, const C: usize, const R: usize> {
    inner: [[usize; C]; R],
    _dist: PhantomData<D>,
}

impl<D: Distance, const C: usize, const R: usize> Default for TileBoard<D, C, R> {
    fn default() -> Self {
        let mut inner = [[0; C]; R];

        let mut i = 1;
        for row in inner.iter_mut() {
            for item in row.iter_mut() {
                *item = i;
                i += 1;
            }
        }
        inner[R - 1][C - 1] = 0;

        Self {
            inner,
            _dist: PhantomData,
        }
    }
}

impl<D: Distance, const C: usize, const R: usize> TileBoard<D, C, R> {
    pub fn shuffled(shuffles: usize) -> Self {
        let mut res: TileBoard<D, C, R> = TileBoard::default();

        let (mut pos_x, mut pos_y) = (C - 1, R - 1);

        let mut r = rand::thread_rng();
        let mut move_options: ArrayVec<(usize, usize), 4> = ArrayVec::new();
        for _ in 0..shuffles {
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
        }

        res
    }
}

impl<D: Distance, const C: usize, const R: usize> State for TileBoard<D, C, R> {
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

    fn finished(&self) -> bool {
        let expected: TileBoard<D, C, R> = TileBoard::default();
        self.inner == expected.inner
    }

    fn score(&self) -> f64 {
        let mut tot = 0f64;
        let mut pos = vec![0; C * R];
        for i in 0..(C * R) {
            let val = self.inner[i % R][i / R];
            pos[val] = i;
        }

        for (real, found) in pos.iter().enumerate() {
            let real = (real / R, real % R);
            let found = (found / R, found % R);
            tot += D::distance(real, found);
        }

        tot / (C * R) as f64
    }
}

impl<D: Distance, const C: usize, const R: usize> Debug for TileBoard<D, C, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.inner {
            write!(f, "\n{:?}", row)?;
        }

        std::fmt::Result::Ok(())
    }
}
