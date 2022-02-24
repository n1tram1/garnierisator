use super::LogicGate;

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

    pub fn add_inputs(self, inputs: Vec<String>) -> Self {
        inputs.iter().fold(self, |b, input| b.add_input(input))
    }

    pub fn add_output(mut self, output: &str) -> Self {
        self.outputs.push(String::from(output));

        self
    }

    pub fn add_outputs(self, outputs: Vec<String>) -> Self {
        outputs.iter().fold(self, |b, output| b.add_output(output))
    }

    pub fn add_logic_gate(mut self, gate: LogicGate) -> Self {
        self.gates.push(gate);

        self
    }

    pub fn add_logic_gates(self, logic_gates: Vec<LogicGate>) -> Self {
        logic_gates.into_iter().fold(self, |b, logic_gate| b.add_logic_gate(logic_gate))
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

