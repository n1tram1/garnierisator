use super::Model;

pub struct Blif {
    models: Vec<Model>,
}

impl Blif {
    pub fn new(models: Vec<Model>) -> Self {
        Self { models }
    }
}
