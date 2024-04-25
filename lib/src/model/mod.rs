use crate::ast::Ast;

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Model<'a> {
    pub checked_state_ids: HashSet<String>,
    pub ast: &'a Ast,
}

impl<'a> Model<'a> {
    pub fn new(ast: &'a Ast) -> Model {
        Model {
            checked_state_ids: HashSet::new(),
            ast,
        }
    }

    pub fn initial_states(&self) -> Vec<State> {
        vec![State::new()]
    }

    pub fn check(self, _state: State) {
        println!("Checking state")
    }

    pub fn adjacent_states(&self, _state: &State) -> Vec<State> {
        vec![State::new()]
    }
}

#[derive(Debug)]
pub struct State {
    pub vars: HashMap<String, IntStateVariable>,
}

impl State {
    pub fn new() -> State {
        State {
            vars: HashMap::new(),
        }
    }

    pub fn id(&self) -> String {
        self.vars
            .iter()
            .map(|(k, v)| format!("{}={}", k, v.val))
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn add_var(&mut self, var: IntStateVariable) {
        self.vars.insert(var.id.clone(), var);
    }

    pub fn get_var(&self, id: &str) -> Option<&IntStateVariable> {
        self.vars.get(id)
    }
}

#[derive(Debug)]
pub struct IntStateVariable {
    pub id: String,
    pub val: i32,
}
