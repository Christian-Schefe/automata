use crate::dea::DEA;
use crate::nea::NEA;

mod dea;
mod nea;

fn main() {
    // let mut dea = DEA::from_transitions("q0", vec![("q0", "q1", "0"), ("q1", "q2", "1")]);
    // dea.add_final_state("q2");

    // println!("{}", dea.simulate_str("0 1 0"));
    // println!("{}", dea.simulate_str("0 1"));

    let mut nea = NEA::from_transitions(
        vec!["q0"],
        vec![("q0", "q0", "0"), ("q0", "q0", "1"), ("q0", "q1", "1"), ("q1", "q2", "0"), ("q1", "q2", "1"), ("q2", "q3", "0"), ("q2", "q3", "1")],
    );
    nea.add_final_state("q3");

    println!("{}", nea.simulate_str("0 1 0"));
    println!("{}", nea.simulate_str("0 1 1"));
    println!("{}", nea.simulate_str("0 1 0 1"));
    println!("{}", nea.simulate_str("0 1 1 1"));
    println!("{}", nea.simulate_str("0 1 1 0"));
    println!("{}", nea.simulate_str("0 1"));

    println!("");
    let dea = nea.to_dea();
    
    println!("{}", dea.simulate_str("0 1 0"));
    println!("{}", dea.simulate_str("0 1 1"));
    println!("{}", dea.simulate_str("0 1 0 1"));
    println!("{}", dea.simulate_str("0 1 1 1"));
    println!("{}", dea.simulate_str("0 1 1 0"));
    println!("{}", dea.simulate_str("0 1"));
}
