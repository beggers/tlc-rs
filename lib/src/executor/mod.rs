use crate::model::{Model, State};

use std::collections::{HashSet, VecDeque};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutorError {
    #[error("Unimplemented")]
    Unimplemented,
}

#[derive(Debug)]
pub struct Executor<'a> {
    model: &'a Model<'a>,
    checked_state_ids: HashSet<String>,
    to_check: VecDeque<State>,
}

impl<'a> Executor<'a> {
    pub fn new(model: &'a Model) -> Executor<'a> {
        Executor {
            model,
            checked_state_ids: HashSet::new(),
            to_check: VecDeque::new(),
        }
    }

    pub fn check(&mut self) {
        println!("Executing model");
        println!("{:?}", self.model);
        println!("{:?}", self.checked_state_ids);
        for state in self.model.initial_states() {
            self.to_check.push_back(state);
        }
        println!("Found initial states: {:?}", self.to_check);
        while let Some(state) = self.to_check.pop_front() {
            println!("Checking state: {:?}", state);
            if self.checked_state_ids.contains(&state.id()) {
                println!("Already checked state: {:?}", state.id());
                continue;
            }
            self.checked_state_ids.insert(state.id());
            for next_state in self.model.adjacent_states(&state) {
                self.to_check.push_back(next_state);
            }
        }
        println!("Done")
    }
}
