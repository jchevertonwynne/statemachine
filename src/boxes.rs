use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use crate::{
    sharedlist::SharedList,
    traits::{State, StateBox},
};

pub struct BFSBox<S: State> {
    inner: VecDeque<(S, SharedList<S>)>,
}

pub struct DFSBox<S: State> {
    inner: Vec<(S, SharedList<S>)>,
}

#[derive(PartialEq, Eq)]
struct ScoredEntry<S: State> {
    state: S,
    history: SharedList<S>,
}

impl<S: State> PartialOrd for ScoredEntry<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: State> Ord for ScoredEntry<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.state.score() + self.history.len() as f64;
        let b = other.state.score() + other.history.len() as f64;
        a.partial_cmp(&b).unwrap()
    }
}

pub struct AStarBox<S: State> {
    inner: BinaryHeap<Reverse<ScoredEntry<S>>>,
}

impl<S: State> StateBox<S> for BFSBox<S> {
    fn init(state: S) -> Self {
        let mut v = VecDeque::new();
        v.push_back((state, SharedList::new()));
        Self { inner: v }
    }

    fn insert(&mut self, state: S, history: SharedList<S>) {
        self.inner.push_back((state, history));
    }

    fn pop(&mut self) -> Option<(S, SharedList<S>)> {
        self.inner.pop_front()
    }
}

impl<S: State> StateBox<S> for DFSBox<S> {
    fn init(state: S) -> Self {
        let v = vec![(state, SharedList::new())];
        Self { inner: v }
    }

    fn insert(&mut self, state: S, history: SharedList<S>) {
        self.inner.push((state, history));
    }

    fn pop(&mut self) -> Option<(S, SharedList<S>)> {
        self.inner.pop()
    }
}

impl<S: State> StateBox<S> for AStarBox<S> {
    fn init(state: S) -> Self {
        let mut v = BinaryHeap::new();
        v.push(Reverse(ScoredEntry {
            state,
            history: SharedList::new(),
        }));
        Self { inner: v }
    }

    fn insert(&mut self, state: S, history: SharedList<S>) {
        self.inner.push(Reverse(ScoredEntry { state, history }))
    }

    fn pop(&mut self) -> Option<(S, SharedList<S>)> {
        self.inner.pop().map(|i| i.0).map(|i| (i.state, i.history))
    }
}
