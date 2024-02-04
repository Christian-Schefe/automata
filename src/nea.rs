use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use crate::dea::DEA;

pub struct NEA<ST, LT> {
    pub alphabet: HashSet<LT>,
    pub states: HashSet<ST>,
    pub delta: HashMap<(ST, LT), HashSet<ST>>,
    pub start_states: HashSet<ST>,
    pub final_states: HashSet<ST>,
}

impl<ST, LT> NEA<ST, LT>
where
    ST: Eq + Hash + Clone + Ord,
    LT: Eq + Hash + Clone,
{
    pub fn new<T: Into<ST>>(start_states: impl IntoIterator<Item = T>) -> Self {
        let start_states_str = start_states.into_iter().map(T::into).collect();
        NEA {
            alphabet: HashSet::new(),
            states: HashSet::new(),
            delta: HashMap::new(),
            start_states: start_states_str,
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

        let entry = self.delta.entry(key).or_insert(HashSet::new());
        entry.insert(to_str);
    }

    pub fn from_transitions<T: Into<ST>, S: Into<LT>>(
        start_states: impl IntoIterator<Item = T>,
        final_states: impl IntoIterator<Item = T>,
        transitions: Vec<(T, T, S)>,
    ) -> Self {
        let mut nea = NEA::new(start_states);
        for (from_state, to_state, letter) in transitions {
            nea.add_transition(from_state, to_state, letter);
        }
        for final_state in final_states {
            nea.add_final_state(final_state);
        }
        nea
    }

    pub fn simulate<T: Into<ST>, S: Into<LT>>(
        &self,
        states: impl IntoIterator<Item = T>,
        word: impl IntoIterator<Item = S>,
    ) -> Option<HashSet<ST>> {
        let mut cur_states: HashSet<ST> = states.into_iter().map(T::into).collect();

        for letter in word {
            cur_states = self.get_new_states(cur_states, letter);
            if cur_states.len() == 0 {
                return None;
            }
        }
        Some(cur_states)
    }

    pub fn accepts<S: Into<LT>>(&self, word: impl IntoIterator<Item = S>) -> bool {
        let state = self.simulate(self.start_states.clone(), word);

        state.is_some_and(|x| x.iter().any(|e| self.final_states.contains(e)))
    }

    pub fn to_dea<FN>(&self, state_transformer: FN) -> DEA<ST, LT>
    where
        FN: Fn(&HashSet<ST>) -> ST,
    {
        let start_state = state_transformer(&self.start_states);
        let mut dea = DEA::<ST, LT>::new(start_state.clone());

        let mut visited_states: HashSet<ST> = HashSet::new();
        visited_states.insert(start_state);

        let mut current_states = VecDeque::new();
        current_states.push_back(self.start_states.clone());

        while let Some(cur) = current_states.pop_front() {
            let cur_str = state_transformer(&cur);
            if cur.iter().any(|x| self.final_states.contains(x)) {
                dea.add_final_state(cur_str.clone())
            }
            for letter in self.alphabet.iter() {
                let new_states = self.get_new_states(cur.clone(), letter.clone());
                let state_str = state_transformer(&new_states);

                dea.add_transition(cur_str.clone(), state_str.clone(), letter.clone());

                if visited_states.insert(state_str) {
                    current_states.push_back(new_states);
                }
            }
        }

        dea
    }

    fn get_new_states<T: Into<ST>, S: Into<LT>>(
        &self,
        states: impl IntoIterator<Item = T>,
        letter: S,
    ) -> HashSet<ST> {
        let mut next_states_set = HashSet::new();
        let letter_str: LT = letter.into();
        for state in states {
            let key = (state.into(), letter_str.clone());
            if let Some(next_states) = self.delta.get(&key) {
                next_states_set.extend(next_states.to_owned())
            }
        }
        next_states_set
    }
}
