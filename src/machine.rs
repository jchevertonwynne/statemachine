use std::collections::HashSet;

use crate::traits::{Solver, State, StateBox};

#[derive(Clone)]
pub struct Machine<S: State> {
    init_state: S,
}

impl<S: State> Machine<S> {
    pub fn new(init_state: S) -> Self {
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
                let new_history = history.push(state.clone());
                if next_state.finished() {
                    new_history.push(next_state);
                    println!("found after {} iterations", i);
                    return Some(new_history.into());
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
                let new_history = history.push(state.clone());
                if next_state.finished() {
                    new_history.push(next_state.clone());
                    results.push(new_history.clone().into());
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
