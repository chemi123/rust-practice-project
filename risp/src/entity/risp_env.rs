use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct RispEnv {
    data: HashMap<String, RispEnv>,
}

impl RispEnv {
    pub fn new() -> RispEnv {
        RispEnv {
            data: HashMap::new(),
        }
    }
}
