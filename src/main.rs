use crate::dea::DEA;
use crate::nea::NEA;

mod dea;
mod nea;

fn main() {
    let dea = DEA::<String, String>::from_transitions(
        "q0",
        ["q2"],
        [("q0", "q1", "0"), ("q1", "q2", "1")],
    );

    println!("Alphabet: {:?}", dea.alphabet);
    println!("States: {:?}", dea.states);
    println!("Delta: {:?}", dea.delta);
    println!("Start State: {:?}", dea.start_state);
    println!("Final States: {:?}", dea.final_states);

    println!("{}", dea.accepts("0 1 0".split_ascii_whitespace()));
    println!("{}", dea.accepts("0 1".split_ascii_whitespace()));

    let nea = NEA::<String, String>::from_transitions(
        ["q0"],
        ["q3"],
        vec![
            ("q0", "q0", "0"),
            ("q0", "q0", "1"),
            ("q0", "q1", "1"),
            ("q1", "q2", "0"),
            ("q1", "q2", "1"),
            ("q2", "q3", "0"),
            ("q2", "q3", "1"),
        ],
    );

    println!("{}", nea.accepts("0 1 0".split_ascii_whitespace()));
    println!("{}", nea.accepts("0 1 1".split_ascii_whitespace()));
    println!("{}", nea.accepts("0 1 0 1".split_ascii_whitespace()));
    println!("{}", nea.accepts("0 1 1 1".split_ascii_whitespace()));
    println!("{}", nea.accepts("0 1 1 0".split_ascii_whitespace()));
    println!("{}", nea.accepts("0 1".split_ascii_whitespace()));

    println!("");
    let dea = nea.to_dea(|x| x.iter().cloned().reduce(|y, a| (a + &y)).unwrap());

    println!("{}", dea.accepts("0 1 0".split_ascii_whitespace()));
    println!("{}", dea.accepts("0 1 1".split_ascii_whitespace()));
    println!("{}", dea.accepts("0 1 0 1".split_ascii_whitespace()));
    println!("{}", dea.accepts("0 1 1 1".split_ascii_whitespace()));
    println!("{}", dea.accepts("0 1 1 0".split_ascii_whitespace()));
    println!("{}", dea.accepts("0 1".split_ascii_whitespace()));
}
