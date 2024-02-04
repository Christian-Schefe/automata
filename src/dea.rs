use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::nea::NEA;

pub struct DEA<ST, LT> {
    pub alphabet: HashSet<LT>,
    pub states: HashSet<ST>,
    pub delta: HashMap<(ST, LT), ST>,
    pub start_state: ST,
    pub final_states: HashSet<ST>,
}

impl<ST, LT> DEA<ST, LT>
where
    ST: Eq + Hash + Clone,
    LT: Eq + Hash + Clone,
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
            if let Some(next_state) = self.delta.get(&(cur_state, letter_str)) {
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
}
