use std::fmt::Debug;

use statemachine::{boxes::BFSBox, hanoi::Hanoi, machine::Machine, traits::{Solver, State, StateBox}};

fn main() {
    let state: Hanoi<12> = Hanoi::new();
    let solved = Hanoi::solved();

    println!("{:?}", state);
    let machine = Machine::new(state, solved);

    run_and_report::<_, BFSBox<_>, _>(machine);
}

fn run_and_report<S: State + Debug, SB: StateBox<S>, SO: Solver<S>>(solver: SO) {
    let solution = Solver::<_>::find_one_with_checks::<SB>(solver);
    if let Some((history, turns)) = solution {
        println!(
            "{}: took {} turns to find a solution of length {}",
            std::any::type_name::<SB>(),
            turns,
            history.len()
        );
    } else {
        println!("no solution found 😥");
    }
}
