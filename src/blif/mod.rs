use std::collections::HashSet;

pub mod parser;

#[derive(Debug, PartialEq, Eq, Hash)]
enum InputValue {
    /// "1"
    Uncomplemented,
    /// "0"
    Complemented,
    /// "-"
    NotUsed,
}

#[derive(Debug, PartialEq)]
pub struct LogicGate {
    inputs: Vec<String>,
    output: String,
    truth_values: HashSet<Vec<InputValue>>,
}

pub struct LogicGateBuilder {
    inputs_output: Vec<String>,
    truth_values: Vec<Vec<InputValue>>,
}

impl LogicGateBuilder {
    pub fn new() -> Self {
        Self {
            inputs_output: Vec::new(),
            truth_values: Vec::new(),
        }
    }

    pub fn add_io(mut self, name: &str) -> Self {
        self.inputs_output.push(String::from(name));

        self
    }

    pub fn add_truth_value(mut self, input_plane: Vec<InputValue>) -> Self {
        self.truth_values.push(input_plane);

        self
    }

    pub fn build(mut self) -> Result<LogicGate, &'static str> {
        if self.inputs_output.len() < 1 {
            return Err("logic gates must have at least one output");
        }

        let output = self.inputs_output.pop().unwrap();
        let inputs = self.inputs_output;

        let input_plane_width = inputs.len();

        if self.truth_values.iter().find(|value| value.len() != input_plane_width).is_some() {
            return Err("input planes must be of the same width as the amount of inputs");
        }

        let truth_values: HashSet<Vec<InputValue>> = HashSet::from_iter(self.truth_values);

        let logic_gate = LogicGate {
            inputs,
            output,
            truth_values,
        };

        Ok(logic_gate)
    }
}

pub struct Model {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,

    gates: Vec<LogicGate>,
}

pub struct Blif {
    models: Vec<Model>,
}
