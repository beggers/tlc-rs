use crate::ast::Ast;

use std::collections::HashMap;

// A TLA+ model. Produced by the Analyzer and consumed by the Executor -- holds
// overall model state, transitions, etc.
#[derive(Debug)]
pub struct Model<'a> {
    // Does this module EXTEND NATURALS?
    _naturals: bool,
    pub ast: &'a Ast<'a>,
}

impl<'a> Model<'a> {
    pub fn new(ast: &'a Ast) -> Model<'a> {
        Model {
            _naturals: false,
            ast,
        }
    }

    pub fn initial_states(&self) -> Vec<State> {
        vec![State::new()]
    }

    pub fn check(self, _state: State) -> bool {
        println!("Checking state");
        true
    }

    pub fn adjacent_states(&self, _state: &State) -> Vec<State> {
        vec![State::new()]
    }
}

// A single execution state of the spec wherein each variable has a single
// concrete value.
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
