use statemachine::{boxes::{AStarBox, BFSBox, StaggeredBox}, distances::{Euclidian, Manhattan}, machine::Machine, tileboard::TileBoard, traits::Solver};

fn main() {
    let state: TileBoard<3, 3> = TileBoard::shuffled(100000);
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
        Solver::<_, AStarBox<_, Euclidian>>::find_one(machine).unwrap().len()
    );
}
