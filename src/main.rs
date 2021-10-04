use statemachine::{
    boxes::{AStarBox, BFSBox},
    distances::Euclidian,
    machine::Machine,
    tileboard::TileBoard,
    traits::Solver,
};

fn main() {
    let state: TileBoard<Euclidian, 3, 3> = TileBoard::shuffled(100);
    println!("{:?}", state);
    let machine = Machine::new(state);
    println!(
        "solution is {} moves long",
        Solver::<_, BFSBox<_>>::find_one(machine.clone())
            .unwrap()
            .len()
    );
    println!(
        "solution is {} moves long",
        Solver::<_, AStarBox<_>>::find_one(machine)
            .unwrap()
            .len()
    );
    // Solver::<_, DFSBox<_>>::find_one(Machine::new(state.clone())).unwrap();
}
