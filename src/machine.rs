use std::collections::HashSet;

use crate::traits::{Solver, State, StateBox};

#[derive(Clone)]
pub struct Machine<S: State> {
    init_state: S,
    complete_state: S,
}

impl<S: State> Machine<S> {
    pub fn new(init_state: S, complete_state: S) -> Self {
        Machine {
            init_state,
            complete_state,
        }
    }
}

impl<S: State> Solver<S> for Machine<S> {
    fn find_one_with_checks<SB: StateBox<S>>(self) -> Option<(Vec<S>, usize)> {
        if self.init_state == self.complete_state {
            return Some((vec![self.init_state], 0));
        }

        let mut checks = 0;
        let mut seen = HashSet::new();
        let mut unprocessed_states = SB::init(self.init_state);
        while let Some((state, history)) = unprocessed_states.pop() {
            checks += 1;
            let next_states = state.next();
            for next_state in next_states {
                let new_history = history.push(state.clone());
                if next_state == self.complete_state {
                    let complete_history = new_history.push(next_state.clone());
                    return Some((complete_history.into(), checks));
                }
                if !seen.contains(&next_state) {
                    seen.insert(next_state.clone());
                    unprocessed_states.insert(next_state, new_history);
                }
            }
        }

        None
    }

    fn find_one<SB: StateBox<S>>(self) -> Option<Vec<S>> {
        Solver::<S>::find_one_with_checks::<SB>(self).map(|(res, _)| res)
    }

    fn find_all<SB: StateBox<S>>(self) -> Vec<Vec<S>> {
        let mut results = Vec::new();

        if self.init_state == self.complete_state {
            results.push(vec![self.init_state.clone()]);
        }

        let mut seen = HashSet::new();
        let mut unprocessed_states = SB::init(self.init_state);
        while let Some((state, history)) = unprocessed_states.pop() {
            let next_states = state.next();
            for next_state in next_states {
                let new_history = history.push(state.clone());
                if next_state == self.complete_state {
                    let complete_history = new_history.push(next_state.clone());
                    results.push(complete_history.into());
                }
                if !seen.contains(&next_state) {
                    seen.insert(next_state.clone());
                    unprocessed_states.insert(next_state, new_history);
                }
            }
        }

        results
    }
}
