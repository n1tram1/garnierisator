use super::Model;

use crate::simulation::Simulable;

use std::collections::HashSet;

pub struct Blif {
    models: Vec<Model>,
}

impl Blif {
    pub fn new(models: Vec<Model>) -> Self {
        Self { models }
    }
}

impl Simulable for Blif {
    fn get_inputs(&self) -> HashSet<String> {
        unimplemented!()
    }

    fn children(&self) -> Vec<Box<dyn Simulable>> {
        vec![
            Box::new(self.models.last().unwrap().clone()) as Box<dyn Simulable>
        ]
    }
}
