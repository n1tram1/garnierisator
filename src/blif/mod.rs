use std::collections::HashSet;

pub mod parser;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum InputValue {
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

#[derive(Debug, PartialEq)]
pub struct LogicGate {
    inputs: Vec<String>,
    output: String,
    truth_values: HashSet<Vec<InputValue>>,
}

#[derive(PartialEq, Debug)]
pub struct LogicGateBuilder {
    inputs: Vec<String>,
    output: Option<String>,
    truth_values: Vec<Vec<InputValue>>,
}

impl LogicGateBuilder {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            output: None,
            truth_values: Vec::new(),
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

    pub fn add_truth_value(mut self, input_plane: Vec<InputValue>) -> Self {
        self.truth_values.push(input_plane);

        self
    }

    fn all_truth_values_have_width(&self, width: usize) -> bool {
        self.truth_values.iter().find(|row| row.len() != width).is_none()
    }

    pub fn build(mut self) -> Result<LogicGate, &'static str> {
        if self.output.is_none() {
            return Err("logic gates must have at least one output");
        }

        let input_plane_width = self.inputs.len();

        if !self.all_truth_values_have_width(input_plane_width) {
            return Err("input plane must be of the same width as the amount of inputs");
        }

        let truth_values: HashSet<Vec<InputValue>> = HashSet::from_iter(self.truth_values);

        let logic_gate = LogicGate {
            inputs: self.inputs,
            output: self.output.unwrap(),
            truth_values,
        };

        Ok(logic_gate)
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

    pub fn add_logic_gates(mut self, gates: Vec<LogicGate>) -> Self {
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
