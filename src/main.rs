use std::fmt::Debug;

use statemachine::{
    boxes::AStarBox,
    distances::Euclidian,
    machine::Machine,
    tileboard::{Coord, TileBoard},
    traits::{Solver, State, StateBox},
};

fn main() {
    let state: TileBoard<4, 2> = TileBoard::shuffled(100000);
    let solved = TileBoard::default();
    println!("{:?}", state);
    let machine = Machine::new(state, solved);

    // run_and_report::<_, BFSBox<_>, _>(machine.clone());
    // run_and_report::<_, AStarBox<_, Manhattan, Coord>, _>(machine.clone());
    run_and_report::<_, AStarBox<_, Euclidian, Coord>, _>(machine);
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
        println!("{:?}", history);
    } else {
        println!("no solution found 😥");
    }
}
