use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::nea::NEA;

#[derive(Clone)]
pub struct DEA<ST, LT> {
    pub alphabet: HashSet<LT>,
    pub states: HashSet<ST>,
    pub delta: HashMap<(ST, LT), ST>,
    pub start_state: ST,
    pub final_states: HashSet<ST>,
}

impl<ST, LT> DEA<ST, LT>
where
    ST: Eq + Hash + Clone + Ord + Debug,
    LT: Eq + Hash + Clone + Debug,
{
    pub fn new<T: Into<ST>>(start_state: T) -> Self {
        DEA {
            alphabet: HashSet::new(),
            states: HashSet::new(),
            delta: HashMap::new(),
            start_state: start_state.into(),
            final_states: HashSet::new(),
        }
    }

    pub fn add_final_state<T: Into<ST>>(&mut self, state: T) {
        let state_str = state.into();
        self.final_states.insert(state_str);
    }

    pub fn add_transition<T: Into<ST>, S: Into<LT>>(
        &mut self,
        from_state: T,
        to_state: T,
        letter: S,
    ) {
        let from_str: ST = from_state.into();
        let to_str: ST = to_state.into();
        let letter_str: LT = letter.into();

        self.alphabet.insert(letter_str.clone());
        self.states.insert(from_str.clone());
        self.states.insert(to_str.clone());

        let key = (from_str, letter_str);

        self.delta.insert(key, to_str);
    }

    pub fn from_transitions<T: Into<ST>, S: Into<LT>>(
        start_state: T,
        final_states: impl IntoIterator<Item = T>,
        transitions: impl IntoIterator<Item = (T, T, S)>,
    ) -> Self {
        let mut dea = DEA::new(start_state);
        for (from_state, to_state, letter) in transitions {
            dea.add_transition(from_state, to_state, letter);
        }
        for final_state in final_states {
            dea.add_final_state(final_state);
        }
        dea
    }

    pub fn simulate<T: Into<ST>, S: Into<LT>>(
        &self,
        state: T,
        word: impl IntoIterator<Item = S>,
    ) -> Option<ST> {
        let mut cur_state = state.into();

        for letter in word {
            let letter_str: LT = letter.into();
            if let Some(next_state) = self.get_new_state(cur_state, letter_str) {
                cur_state = next_state.clone();
            } else {
                return None;
            }
        }

        Some(cur_state)
    }

    pub fn accepts<S: Into<LT>>(&self, word: impl IntoIterator<Item = S>) -> bool {
        let state = self.simulate(self.start_state.clone(), word);

        state.is_some_and(|x| self.final_states.contains(&x))
    }

    pub fn to_nea<FN>(self) -> NEA<ST, LT>
    where
        FN: Fn(&HashSet<ST>) -> ST,
    {
        NEA {
            alphabet: self.alphabet,
            states: self.states,
            delta: self
                .delta
                .into_iter()
                .map(|(x, y)| (x, HashSet::from([y])))
                .collect(),
            final_states: self.final_states,
            start_states: HashSet::from([self.start_state]),
        }
    }

    fn get_new_state<T: Into<ST>, S: Into<LT>>(&self, state: T, letter: S) -> Option<ST> {
        self.delta.get(&(state.into(), letter.into())).cloned()
    }

    pub fn minimize(&self) -> Self {
        let mut marked_pairs = HashSet::<(ST, ST)>::new();
        let mut unmarked_pairs = Vec::<(ST, ST)>::new();
        let mut ordered_states: Vec<ST> = self.states.iter().cloned().collect();
        ordered_states.sort();

        for i in 0..ordered_states.len() - 1 {
            for j in 1..ordered_states.len() {
                if i == j {
                    continue;
                }
                let i_state = &ordered_states[i];
                let j_state = &ordered_states[j];

                if (self.final_states.contains(i_state) && !self.final_states.contains(j_state))
                    || (self.final_states.contains(j_state) && !self.final_states.contains(i_state))
                {
                    marked_pairs.insert((i_state.clone(), j_state.clone()));
                } else {
                    unmarked_pairs.push((i_state.clone(), j_state.clone()));
                }
            }
        }

        let mut changed = true;
        while changed {
            changed = false;
            let old_pairs = unmarked_pairs;
            unmarked_pairs = Vec::<(ST, ST)>::new();

            for pair in old_pairs {
                let mut was_marked = false;

                for letter in self.alphabet.iter() {
                    let next_i = self.get_new_state(pair.0.clone(), letter.clone());
                    let next_j = self.get_new_state(pair.1.clone(), letter.clone());

                    let is_marked = match (next_i, next_j) {
                        (Some(i), Some(j)) => {
                            (self.final_states.contains(&i) && !self.final_states.contains(&j))
                                || (self.final_states.contains(&j)
                                    && !self.final_states.contains(&i))
                                || marked_pairs.contains(&(i, j))
                        }
                        (None, Some(j)) => self.final_states.contains(&j),
                        (Some(i), None) => self.final_states.contains(&i),
                        (None, None) => false,
                    };

                    if is_marked {
                        was_marked = true;
                        break;
                    }
                }
                if was_marked {
                    changed = true;
                    marked_pairs.insert(pair);
                } else {
                    unmarked_pairs.push(pair);
                }
            }
        }
        println!("{:?}\n{:?}", marked_pairs, unmarked_pairs);
        self.clone()
    }

    pub fn combine_states<T: Into<ST>>(
        &mut self,
        state1: T,
        state2: T,
        combiner: fn(ST, ST) -> ST,
    ) {
        let state1c = state1.into();
        let state2c = state2.into();
        let new_state = combiner(state1c.clone(), state2c.clone());
        let mut transitions = Vec::new();

        for ((s0, l), s1) in self.delta.iter() {
            let s0c = s0.clone();
            let s1c = s1.clone();
            let lc = l.clone();
            if (s0c == state1c || s0c == state2c) && (s1c == state1c || s1c == state2c) {
                transitions.push((new_state.clone(), new_state.clone(), lc))
            } else if s0c == state1c || s0c == state2c {
                transitions.push((new_state.clone(), s1c, lc))
            } else if s1c == state1c || s1c == state2c {
                transitions.push((s0c, new_state.clone(), lc))
            } else {
                transitions.push((s0c, s1c, lc))
            }
        }
        let final_states = self
            .final_states
            .iter()
            .cloned()
            .map(|x| if x != state1c && x != state2c {x} else {new_state.clone()});
        let start_state = if self.start_state != state1c && self.start_state != state2c {
            self.start_state.clone()
        } else {
            new_state.clone()
        };
        *self = DEA::from_transitions(start_state, final_states, transitions);
    }
}

impl<ST, LT> Display for DEA<ST, LT>
where
    ST: Debug + Display,
    LT: Debug + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Alphabet: {:?}\nStates: {:?}\nDelta: {:?}\nStart State: {}\nFinal States: {:?}",
            self.alphabet, self.states, self.delta, self.start_state, self.final_states
        ))
    }
}
