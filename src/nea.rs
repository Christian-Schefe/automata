use std::collections::{HashMap, HashSet, VecDeque};

use crate::dea::DEA;

pub struct NEA {
    alphabet: HashSet<String>,
    states: HashSet<String>,
    delta: HashMap<String, HashMap<String, HashSet<String>>>,
    start_states: HashSet<String>,
    final_states: HashSet<String>,
}

impl NEA {
    pub fn new<T: Into<String>>(start_states: HashSet<T>) -> Self {
        let start_states_str = start_states.into_iter().map(T::into).collect();
        NEA {
            alphabet: HashSet::new(),
            states: HashSet::new(),
            delta: HashMap::new(),
            start_states: start_states_str,
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
            if !map.contains_key(&letter_str) {
                map.insert(letter_str.clone(), HashSet::new());
            }
            if let Some(set) = map.get_mut(&letter_str) {
                set.insert(to_str);
            }
        }
    }

    pub fn from_transitions<T: Into<String>>(
        start_states: impl IntoIterator<Item = T>,
        transitions: Vec<(T, T, T)>,
    ) -> Self {
        let mut nea = NEA::new(start_states.into_iter().map(T::into).collect());
        for (from_state, to_state, letter) in transitions {
            nea.add_transition(from_state, to_state, letter);
        }
        nea
    }

    pub fn simulate<T: Into<String>>(&self, word: Vec<T>) -> bool {
        let mut cur_states = self.start_states.clone();

        for letter in word {
            let letter_str: String = letter.into();

            cur_states = self.get_new_states(cur_states.iter(), &letter_str);
            if cur_states.len() == 0 {
                return false;
            }
        }

        for state in cur_states {
            if self.final_states.contains(&state) {
                return true;
            }
        }
        false
    }

    pub fn simulate_str<T: Into<String>>(&self, word: T) -> bool {
        let word_str: String = word.into();
        let split_word = word_str.trim().split_whitespace();
        self.simulate(split_word.collect())
    }

    pub fn to_dea(&self) -> DEA {
        let start_state = state_set_to_state(self.start_states.iter());
        let mut dea = DEA::new(start_state.clone());

        let mut visited_states: HashSet<String> = HashSet::new();
        visited_states.insert(start_state);

        let mut current_states = VecDeque::new();
        current_states.push_back(self.start_states.clone());

        while let Some(cur) = current_states.pop_front() {
            let cur_str = state_set_to_state(cur.iter());
            for letter in self.alphabet.iter() {
                let new_states = self.get_new_states(cur.iter(), letter);
                let state_str = state_set_to_state(new_states.iter());

                dea.add_transition(&cur_str, &state_str, letter);
                if new_states.iter().any(|x| self.final_states.contains(x)) {
                    dea.add_final_state(state_str.clone())
                }

                if visited_states.insert(state_str.clone()) {
                    current_states.push_back(new_states);
                }
            }
        }

        dea
    }

    fn get_new_states<T: Into<String>>(
        &self,
        states: impl Iterator<Item = T>,
        letter: &str,
    ) -> HashSet<String> {
        let mut next_states_set = HashSet::new();

        for state in states {
            if let Some(map) = self.delta.get(&state.into()) {
                if let Some(next_states) = map.get(letter) {
                    next_states_set.extend(next_states.to_owned());
                }
            }
        }
        next_states_set
    }
}

fn state_set_to_state<T: Into<String>>(states: impl Iterator<Item = T>) -> String {
    let mut vec: Vec<String> = states.map(T::into).collect();
    vec.sort();
    vec.join("")
}
