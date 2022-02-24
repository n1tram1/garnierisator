use std::collections::HashSet;

mod logic_gate;
pub use logic_gate::LogicGate;

use logic_gate::LogicGateBuilder;

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

#[derive(PartialEq, Debug)]
pub struct Model {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,

    gates: Vec<LogicGate>,
}

pub struct ModelBuilder {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    gates: Vec<LogicGate>,
}

impl ModelBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            inputs: Vec::new(),
            outputs: Vec::new(),
            gates: Vec::new(),
        }
    }

    pub fn add_input(mut self, input: &str) -> Self {
        self.inputs.push(String::from(input));

        self
    }

    pub fn add_inputs(mut self, inputs: Vec<String>) -> Self {
        inputs.iter().fold(self, |b, input| b.add_input(input))
    }

    pub fn add_output(mut self, output: &str) -> Self {
        self.outputs.push(String::from(output));

        self
    }

    pub fn add_outputs(mut self, outputs: Vec<String>) -> Self {
        outputs.iter().fold(self, |b, output| b.add_output(output))
    }

    pub fn add_logic_gate(mut self, gate: LogicGate) -> Self {
        self.gates.push(gate);

        self
    }

    pub fn add_logic_gates(self, gates: Vec<LogicGate>) -> Self {
        gates.into_iter().fold(self, |b, gate| b.add_logic_gate(gate))
    }

    pub fn build(self) -> Model {
        Model {
            name: self.name,
            inputs: self.inputs,
            outputs: self.outputs,
            gates: self.gates,
        }
    }
}

pub struct Blif {
    models: Vec<Model>,
}
