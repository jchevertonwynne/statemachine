use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    marker::PhantomData,
};

use crate::{
    sharedlist::SharedList,
    traits::{Distance, State, StateBox},
};

pub struct BFSBox<S: State> {
    inner: VecDeque<(S, SharedList<S>)>,
}

pub struct DFSBox<S: State> {
    inner: Vec<(S, SharedList<S>)>,
}

pub struct AStarBox<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> {
    inner: BinaryHeap<Reverse<AStarEntry<S, D, Diff>>>,
}

pub struct StaggeredBox<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> {
    inner: Vec<BinaryHeap<Reverse<ScoredEntry<S, D, Diff>>>>,
}

struct AStarEntry<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> {
    state: S,
    history: SharedList<S>,
    _dist: PhantomData<D>,
}

struct ScoredEntry<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> {
    state: S,
    history: SharedList<S>,
    _dist: PhantomData<D>,
}

impl<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> AStarEntry<S, D, Diff> {
    fn score(&self) -> f64 {
        self.state
            .differences()
            .into_iter()
            .map(|(real, found)| D::distance(real, found))
            .sum::<f64>()
            + self.history.len() as f64
    }
}

impl<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> PartialEq for AStarEntry<S, D, Diff> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.history == other.history
    }
}

impl<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> Eq for AStarEntry<S, D, Diff> {}

impl<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> PartialOrd
    for AStarEntry<S, D, Diff>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> Ord for AStarEntry<S, D, Diff> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.score();
        let b = other.score();
        a.partial_cmp(&b).unwrap()
    }
}

impl<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> ScoredEntry<S, D, Diff> {
    fn score(&self) -> f64 {
        self.state
            .differences()
            .into_iter()
            .map(|(real, found)| D::distance(real, found))
            .sum::<f64>()
    }
}

impl<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> PartialEq
    for ScoredEntry<S, D, Diff>
{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.history == other.history
    }
}

impl<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> Eq for ScoredEntry<S, D, Diff> {}

impl<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> PartialOrd
    for ScoredEntry<S, D, Diff>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> Ord for ScoredEntry<S, D, Diff> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.score();
        let b = other.score();
        a.partial_cmp(&b).unwrap()
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

impl<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> StateBox<S> for AStarBox<S, D, Diff> {
    fn init(state: S) -> Self {
        let mut inner = BinaryHeap::new();
        inner.push(Reverse(AStarEntry {
            state,
            history: SharedList::new(),
            _dist: PhantomData,
        }));
        Self { inner }
    }

    fn insert(&mut self, state: S, history: SharedList<S>) {
        self.inner.push(Reverse(AStarEntry {
            state,
            history,
            _dist: PhantomData,
        }))
    }

    fn pop(&mut self) -> Option<(S, SharedList<S>)> {
        self.inner.pop().map(|i| i.0).map(|i| (i.state, i.history))
    }
}

impl<S: State<Point = Diff>, D: Distance<Point = Diff>, Diff> StateBox<S>
    for StaggeredBox<S, D, Diff>
{
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
        self.inner[history.len()].push(Reverse(ScoredEntry {
            state,
            history,
            _dist: PhantomData,
        }))
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
