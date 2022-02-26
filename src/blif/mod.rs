mod logic_gate;
use logic_gate::LogicGateBuilder;
pub use logic_gate::LogicGate;

mod model;
use model::ModelBuilder;
pub use model::Model;

pub mod parser;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum InputValue {
    /// "1"
    Uncomplemented,
    /// "0"
    Complemented,
    /// "-"
    NotUsed,
}

impl std::convert::TryFrom<char> for InputValue {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::Uncomplemented),
            '0' => Ok(Self::Complemented),
            '-' => Ok(Self::NotUsed),
            _ => Err("expected [0|1|-] to create a InputValue"),
        }
    }
}

impl std::convert::TryFrom<&char> for InputValue {
    type Error = &'static str;

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        Self::try_from(*c)
    }
}

pub struct Blif {
    models: Vec<Model>,
}

use std::collections::HashSet;
use crate::simulation::Simulable ;

impl Simulable for Blif {
    fn get_inputs(&self) -> HashSet<String> {
        unimplemented!()
    }

    fn children(&self) -> Vec<Box<dyn Simulable>> {
        self.models.iter().map(|x| {
            Box::new(x.clone()) as Box<dyn Simulable>
        }).collect()
    }
}
