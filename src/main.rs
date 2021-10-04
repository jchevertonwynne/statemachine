use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
};

use rand::prelude::SliceRandom;

fn main() {
    let state: NineTiles<Euclidian, 10, 2> = NineTiles::new();
    println!("{:?}", state);
    Solver::<_, ScoredBox<_>>::find_one(Machine::new(state.clone())).unwrap();
    // Solver::<_, BFSBox<_>>::find_one(Machine::new(state.clone())).unwrap();
    // Solver::<_, DFSBox<_>>::find_one(Machine::new(state.clone())).unwrap();
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct NineTiles<D: Distance, const C: usize, const R: usize> {
    inner: [[usize; C]; R],
    _dist: PhantomData<D>,
}

impl<D: Distance, const C: usize, const R: usize> Default for NineTiles<D, C, R> {
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

impl<D: Distance, const C: usize, const R: usize> NineTiles<D, C, R> {
    fn new() -> Self {
        let d: NineTiles<D, C, R> = NineTiles::default();
        let mut inner = d.inner;

        let (mut pos_x, mut pos_y) = (C - 1, R - 1);

        let mut r = rand::thread_rng();
        let mut move_options = Vec::new();
        for _ in 0..100000 {
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
            let t = inner[c_y][c_x];
            inner[c_y][c_x] = inner[pos_y][pos_x];
            inner[pos_y][pos_x] = t;
            pos_x = c_x;
            pos_y = c_y;
        }

        Self {
            inner,
            _dist: PhantomData,
        }
    }
}

impl<D: Distance, const C: usize, const R: usize> State for NineTiles<D, C, R> {
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
        let expected: NineTiles<D, C, R> = NineTiles::default();
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

        tot as f64
    }
}

impl<D: Distance, const C: usize, const R: usize> Debug for NineTiles<D, C, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.inner {
            write!(f, "\n{:?}", row)?;
        }

        std::fmt::Result::Ok(())
    }
}

impl<D: Distance, const C: usize, const R: usize> PartialOrd for NineTiles<D, C, R> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<D: Distance, const C: usize, const R: usize> Ord for NineTiles<D, C, R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().partial_cmp(&other.score()).unwrap()
    }
}

trait State: Hash + Eq + Sized + Clone {
    fn next(&self) -> Vec<Self>;
    fn finished(&self) -> bool;
    fn score(&self) -> f64;
}

trait Solver<S: State, SB: StateBox<S>> {
    fn find_one(self) -> Option<Vec<S>>;
    fn find_all(self) -> Vec<Vec<S>>;
}

trait StateBox<S: State> {
    fn init(state: S) -> Self;
    fn insert(&mut self, state: S, history: Vec<S>);
    fn pop(&mut self) -> Option<(S, Vec<S>)>;
}

trait Distance: Clone + Hash + Eq {
    fn distance(a: (usize, usize), b: (usize, usize)) -> f64;
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Manhattan;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Euclidian;

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
        x_diff.powi(2) + y_diff.powi(2)
    }
}

struct BFSBox<S: State> {
    inner: VecDeque<(S, Vec<S>)>,
}

struct DFSBox<S: State> {
    inner: Vec<(S, Vec<S>)>,
}

struct ScoredBox<S: State> {
    inner: BinaryHeap<Reverse<(S, Vec<S>)>>,
}

impl<S: State> StateBox<S> for BFSBox<S> {
    fn init(state: S) -> Self {
        let mut v = VecDeque::new();
        v.push_back((state, Vec::new()));
        Self { inner: v }
    }

    fn insert(&mut self, state: S, history: Vec<S>) {
        self.inner.push_back((state, history));
    }

    fn pop(&mut self) -> Option<(S, Vec<S>)> {
        self.inner.pop_front()
    }
}

impl<S: State> StateBox<S> for DFSBox<S> {
    fn init(state: S) -> Self {
        let v = vec![(state, Vec::new())];
        Self { inner: v }
    }

    fn insert(&mut self, state: S, history: Vec<S>) {
        self.inner.push((state, history));
    }

    fn pop(&mut self) -> Option<(S, Vec<S>)> {
        self.inner.pop()
    }
}

impl<S: State + Ord> StateBox<S> for ScoredBox<S> {
    fn init(state: S) -> Self {
        let mut v = BinaryHeap::new();
        v.push(Reverse((state, Vec::new())));
        Self { inner: v }
    }

    fn insert(&mut self, state: S, history: Vec<S>) {
        self.inner.push(Reverse((state, history)))
    }

    fn pop(&mut self) -> Option<(S, Vec<S>)> {
        self.inner.pop().map(|i| i.0)
    }
}

struct Machine<S: State> {
    init_state: S,
}

impl<S: State> Machine<S> {
    fn new(init_state: S) -> Self {
        Machine { init_state }
    }
}

impl<S: State, SB: StateBox<S>> Solver<S, SB> for Machine<S> {
    fn find_one(self) -> Option<Vec<S>> {
        if self.init_state.finished() {
            return Some(vec![self.init_state]);
        }

        let mut i = 0;
        let mut seen = HashSet::new();
        let mut active = SB::init(self.init_state);
        while let Some((state, history)) = active.pop() {
            i += 1;
            let next_states = state.next();
            for next_state in next_states {
                let mut new_history = history.clone();
                new_history.push(state.clone());
                if next_state.finished() {
                    new_history.push(next_state);
                    println!("found after {} iterations", i);
                    return Some(new_history);
                }
                if !seen.contains(&next_state) {
                    seen.insert(next_state.clone());
                    active.insert(next_state, new_history);
                }
            }
        }
        None
    }

    fn find_all(self) -> Vec<Vec<S>> {
        let mut results = Vec::new();

        if self.init_state.finished() {
            results.push(vec![self.init_state.clone()]);
        }

        let mut seen = HashSet::new();
        let mut active = SB::init(self.init_state);
        while let Some((state, history)) = active.pop() {
            let next_states = state.next();
            for next_state in next_states {
                let mut new_history = history.clone();
                new_history.push(state.clone());
                if next_state.finished() {
                    new_history.push(next_state.clone());
                    results.push(new_history.clone());
                }
                if !seen.contains(&next_state) {
                    seen.insert(next_state.clone());
                    active.insert(next_state, new_history);
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_find_next() {
        let world: NineTiles<Euclidian, 10, 2> = NineTiles::new();
        println!("{:?}", world);
        let zero = world
            .inner
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, item)| (i, j, *item)))
            .filter_map(|(i, j, item)| if item == 0 { Some((i, j)) } else { None })
            .next()
            .unwrap();
        println!("{:?}", zero);
        let next = world.next();
        println!("{:?}", next);
        for n in &next {
            println!("{}", n.score());
        }

        let mut selector = ScoredBox::init(world.clone());
        selector.pop();
        for n in next {
            selector.insert(n, vec![]);
        }
        println!("{}", selector.pop().unwrap().0.score())
    }
}
