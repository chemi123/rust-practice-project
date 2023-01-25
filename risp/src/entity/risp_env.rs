use std::collections::HashMap;

use super::risp_exp::RispExp;

#[derive(Clone)]
pub struct RispEnv {
    data: HashMap<String, RispExp>,
}

impl RispEnv {
    pub fn new(data: HashMap<String, RispExp>) -> RispEnv {
        RispEnv { data }
    }

    pub fn insert(&mut self, key: String, val: RispExp) {
        self.data.insert(key, val);
    }

    pub fn get(&self, key: &String) -> Option<&RispExp> {
        self.data.get(key)
    }
}
