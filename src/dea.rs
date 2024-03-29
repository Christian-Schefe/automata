use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    hash::Hash,
};

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

    fn get_new_state(&self, state: ST, letter: LT) -> Option<ST> {
        self.delta.get(&(state, letter)).cloned()
    }

    pub fn completed(&self, trap_state: impl Into<ST>) -> Self {
        let mut transitions = Vec::new();
        let trap = trap_state.into();
        for state in self.states.iter() {
            for letter in self.alphabet.iter() {
                match self.get_new_state(state.clone(), letter.clone()) {
                    Some(new_state) => transitions.push((state.clone(), new_state, letter.clone())),
                    None => transitions.push((state.clone(), trap.clone(), letter.clone())),
                }
            }
        }

        DEA::from_transitions(
            self.start_state.clone(),
            self.final_states.clone(),
            transitions,
        )
    }

    pub fn minimized(&self, combiner: fn(HashSet<ST>) -> ST) -> Self {
        let mut marked_pairs: HashSet<(ST, ST)> = HashSet::new();
        let mut unmarked_pairs: Vec<(ST, ST)> = Vec::new();
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

        let mut min_dea = self.clone();
        min_dea.combine_state_pairs(unmarked_pairs, combiner);
        min_dea
    }

    fn combine_state_pairs(
        &mut self,
        pairs: impl IntoIterator<Item = (ST, ST)>,
        combiner: fn(HashSet<ST>) -> ST,
    ) {
        let mut neighbours: HashMap<ST, Vec<ST>> = self
            .states
            .iter()
            .map(|x| (x.clone(), Vec::new()))
            .collect();
        for (s0, s1) in pairs {
            neighbours.get_mut(&s0).unwrap().push(s1.clone());
            neighbours.get_mut(&s1).unwrap().push(s0);
        }

        let mut visited: HashSet<ST> = HashSet::new();
        let mut groups: Vec<HashSet<ST>> = Vec::new();
        for state in self.states.iter() {
            if visited.contains(state) {
                continue;
            }

            let mut group: HashSet<ST> = HashSet::new();
            let mut queue: VecDeque<ST> = VecDeque::new();
            queue.push_back(state.clone());

            while let Some(cur_state) = queue.pop_front() {
                group.insert(cur_state.clone());
                for n in neighbours[&cur_state].iter() {
                    if !group.contains(n) {
                        queue.push_back(n.clone());
                    }
                }
            }

            visited.extend(group.iter().cloned());
            groups.push(group);
        }

        for group in groups {
            self.combine_states(group, combiner);
        }
    }

    fn combine_states<T: Into<ST>>(
        &mut self,
        states: impl IntoIterator<Item = T>,
        combiner: fn(HashSet<ST>) -> ST,
    ) {
        let mapped_states: HashSet<ST> = states.into_iter().map(T::into).collect();
        let new_state = combiner(mapped_states.clone());

        let state_map: HashMap<ST, &ST> = self
            .states
            .iter()
            .map(|x| {
                (
                    x.clone(),
                    if mapped_states.contains(x) {
                        &new_state
                    } else {
                        x
                    },
                )
            })
            .collect();

        let mut transitions = Vec::new();

        for ((from_state, letter), to_state) in self.delta.iter() {
            let new_from_state = state_map[from_state];
            let new_to_state = state_map[to_state];

            transitions.push((new_from_state.clone(), new_to_state.clone(), letter.clone()))
        }

        let start_state = state_map[&self.start_state].clone();
        let final_states: HashSet<ST> = self
            .final_states
            .iter()
            .map(|x| state_map[x])
            .cloned()
            .collect();

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
