use std::collections::{HashMap, HashSet};

pub struct DEA {
    alphabet: HashSet<String>,
    states: HashSet<String>,
    delta: HashMap<String, HashMap<String, String>>,
    start_state: String,
    final_states: HashSet<String>,
}

impl DEA {
    pub fn new<T: Into<String>>(start_state: T) -> Self {
        DEA {
            alphabet: HashSet::new(),
            states: HashSet::new(),
            delta: HashMap::new(),
            start_state: start_state.into(),
            final_states: HashSet::new(),
        }
    }

    pub fn add_final_state<T: Into<String>>(&mut self, state: T) {
        let state_str = state.into();
        self.final_states.insert(state_str);
    }

    pub fn add_transition<T: Into<String>>(&mut self, from_state: T, to_state: T, letter: T) {
        let from_str: String = from_state.into();
        let to_str: String = to_state.into();
        let letter_str: String = letter.into();

        self.alphabet.insert(letter_str.clone());
        self.states.insert(from_str.clone());
        self.states.insert(to_str.clone());

        if !self.delta.contains_key(&from_str) {
            self.delta.insert(from_str.clone(), HashMap::new());
        }
        if let Some(map) = self.delta.get_mut(&from_str) {
            map.insert(letter_str, to_str);
        }
    }

    pub fn from_transitions<T: Into<String>>(start_state: T, transitions: Vec<(T, T, T)>) -> Self {
        let mut dea = DEA::new(start_state);
        for (from_state, to_state, letter) in transitions {
            dea.add_transition(from_state, to_state, letter);
        }
        dea
    }

    pub fn simulate<T: Into<String>>(&self, word: Vec<T>) -> bool {
        let mut cur_state = &self.start_state;

        for letter in word {
            let letter_str: String = letter.into();
            if let Some(map) = self.delta.get(cur_state) {
                if let Some(next_state) = map.get(&letter_str) {
                    cur_state = next_state
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        self.final_states.contains(cur_state)
    }

    pub fn simulate_str<T: Into<String>>(&self, word: T) -> bool {
        let word_str: String = word.into();
        let split_word = word_str.trim().split_whitespace();
        self.simulate(split_word.collect())
    }
}
