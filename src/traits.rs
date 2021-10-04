use std::hash::Hash;

use crate::sharedlist::SharedList;

pub trait State: Hash + Eq + Sized + Clone {
    fn next(&self) -> Vec<Self>;
    fn finished(&self) -> bool;
    fn score(&self) -> f64;
}

pub trait Solver<S: State, SB: StateBox<S>> {
    fn find_one(self) -> Option<Vec<S>>;
    fn find_all(self) -> Vec<Vec<S>>;
}

pub trait StateBox<S: State> {
    fn init(state: S) -> Self;
    fn insert(&mut self, state: S, history: SharedList<S>);
    fn pop(&mut self) -> Option<(S, SharedList<S>)>;
}

pub trait Distance: Clone + Hash + Eq {
    fn distance(a: (usize, usize), b: (usize, usize)) -> f64;
}
