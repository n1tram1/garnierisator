use std::collections::HashSet;

use super::InputValue;

type InputVariables = Vec<InputValue>;

#[derive(Debug, PartialEq)]
pub struct LogicGate {
    inputs: Vec<String>,
    output: String,
    truth_table: Vec<(InputVariables, InputValue)>,
}

#[derive(PartialEq, Debug)]
pub struct LogicGateBuilder {
    inputs: Vec<String>,
    output: Option<String>,
    truth_table: Vec<(InputVariables, InputValue)>,
}

impl LogicGateBuilder {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            output: None,
            truth_table: Vec::new(),
        }
    }

    pub fn add_input(mut self, name: &str) -> Self {
        self.inputs.push(String::from(name));

        self
    }

    pub fn set_output(mut self, name: &str) -> Self {
        self.output = Some(String::from(name));

        self
    }

    pub fn add_truth_table_row(mut self, row: (InputVariables, InputValue)) -> Self {
        self.truth_table.push(row);

        self
    }

    fn all_truth_values_have_width(&self, width: usize) -> bool {
        self.truth_table.iter().find(|(inputs, _output)| inputs.len() != width).is_none()
    }

    pub fn build(mut self) -> Result<LogicGate, &'static str> {
        if self.output.is_none() {
            return Err("logic gates must have at least one output");
        }

        let input_plane_width = self.inputs.len();

        if !self.all_truth_values_have_width(input_plane_width) {
            return Err("input plane must be of the same width as the amount of inputs");
        }

        let logic_gate = LogicGate {
            inputs: self.inputs,
            output: self.output.unwrap(),
            truth_table: self.truth_table,
        };

        Ok(logic_gate)
    }
}
