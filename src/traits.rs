use std::hash::Hash;

use crate::sharedlist::SharedList;

pub trait State: Hash + Eq + Sized + Clone {
    type Point;

    fn next(&self) -> Vec<Self>;
    fn differences(&self) -> Vec<(Self::Point, Self::Point)>;
}

pub trait Solver<S: State> {
    fn find_one_with_checks<SB: StateBox<S>>(self) -> Option<(Vec<S>, usize)>;
    fn find_one<SB: StateBox<S>>(self) -> Option<Vec<S>>;
    fn find_all<SB: StateBox<S>>(self) -> Vec<Vec<S>>;
}

pub trait StateBox<S: State> {
    fn init(state: S) -> Self;
    fn insert(&mut self, state: S, history: SharedList<S>);
    fn pop(&mut self) -> Option<(S, SharedList<S>)>;
}

pub trait Distance: Clone + Hash + Eq {
    type Point;

    fn distance(a: Self::Point, b: Self::Point) -> f64;
}
