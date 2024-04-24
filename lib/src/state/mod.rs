use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
    pub id: String,
    pub vars: HashMap<String, IntStateVariable>,
}

impl State {
    pub fn new(id: String) -> State {
        State {
            id,
            vars: HashMap::new(),
        }
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
