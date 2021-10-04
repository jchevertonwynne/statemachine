use std::{cmp::Reverse, collections::{BinaryHeap, VecDeque}, marker::PhantomData};

use crate::{sharedlist::SharedList, traits::{Distance, State, StateBox}};

pub struct BFSBox<S: State> {
    inner: VecDeque<(S, SharedList<S>)>,
}

pub struct DFSBox<S: State> {
    inner: Vec<(S, SharedList<S>)>,
}

pub struct AStarBox<S: State, D: Distance> {
    inner: BinaryHeap<Reverse<AStarEntry<S, D>>>,
}

pub struct StaggeredBox<S: State> {
    inner: Vec<BinaryHeap<Reverse<ScoredEntry<S>>>>,
}

#[derive(PartialEq, Eq)]
struct AStarEntry<S: State, D: Distance> {
    state: S,
    history: SharedList<S>,
    _dist: PhantomData<D>
}

#[derive(PartialEq, Eq)]
struct ScoredEntry<S: State> {
    state: S,
    history: SharedList<S>
}

impl<S: State, D: Distance> PartialOrd for AStarEntry<S, D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: State, D: Distance> Ord for AStarEntry<S, D> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.state.differences();
        let b = other.state.differences();
        let a = {
            let mut res = 0f64;
            for (real, found) in a {
                res += D::distance(real, found);
            }
            res
        };
        let b = {
            let mut res = 0f64;
            for (real, found) in b {
                res += D::distance(real, found);
            }
            res
        };
        a.partial_cmp(&b).unwrap()
    }
}

impl<S: State> PartialOrd for ScoredEntry<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: State> Ord for ScoredEntry<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.state.differences().partial_cmp(&other.state.differences()).unwrap()
    }
}

impl<S: State> StateBox<S> for BFSBox<S> {
    fn init(state: S) -> Self {
        let mut inner = VecDeque::new();
        inner.push_back((state, SharedList::new()));
        Self { inner }
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
        let inner = vec![(state, SharedList::new())];
        Self { inner }
    }

    fn insert(&mut self, state: S, history: SharedList<S>) {
        self.inner.push((state, history));
    }

    fn pop(&mut self) -> Option<(S, SharedList<S>)> {
        self.inner.pop()
    }
}

impl<S: State, D: Distance> StateBox<S> for AStarBox<S, D> {
    fn init(state: S) -> Self {
        let mut inner = BinaryHeap::new();
        inner.push(Reverse(AStarEntry {
            state,
            history: SharedList::new(),
            _dist: PhantomData
        }));
        Self { inner }
    }

    fn insert(&mut self, state: S, history: SharedList<S>) {
        self.inner.push(Reverse(AStarEntry { state, history, _dist: PhantomData }))
    }

    fn pop(&mut self) -> Option<(S, SharedList<S>)> {
        self.inner.pop().map(|i| i.0).map(|i| (i.state, i.history))
    }
}

impl<S: State> StateBox<S> for StaggeredBox<S> {
    fn init(state: S) -> Self {
        let inner = Vec::new();
        let mut res = Self { inner };
        res.insert(state, SharedList::new());
        res
    }

    fn insert(&mut self, state: S, history: SharedList<S>) {
        let required_len = history.len() + 1;
        let actual_len = self.inner.len();
        if actual_len < required_len {
            self.inner.reserve(self.inner.len() - history.len() + 1);
            while self.inner.len() < required_len {
                self.inner.push(BinaryHeap::new());
            }
        }
        self.inner[history.len()].push(Reverse(ScoredEntry { state, history }))
    }

    fn pop(&mut self) -> Option<(S, SharedList<S>)> {
        self.inner
            .iter_mut()
            .filter_map(|b| b.pop())
            .next()
            .map(|i| i.0)
            .map(|i| (i.state, i.history))
    }
}
