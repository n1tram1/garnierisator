use super::LogicGate;

#[derive(PartialEq, Debug, Clone)]
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

use std::collections::HashSet;
use crate::simulation::Simulable;

impl Simulable for Model {
    fn get_inputs(&self) -> HashSet<String> {
        self.inputs.clone().into_iter().collect()
    }

    fn children(&self) -> Vec<Box<dyn Simulable>> {
        self.gates.iter().map(|x| {
            Box::new(x.clone()) as Box<dyn Simulable>
        }).collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::blif::{InputValue, ModelBuilder, LogicGateBuilder};
    use crate::simulation::{SignalState, SignalsBuilder, Simulable};

    #[test]
    fn test_model_not2() {
        let not2 = ModelBuilder::new("not2")
            .add_input("a")
            .add_input("b")
            .add_output("o_a")
            .add_output("o_b")
            .add_logic_gate(
                LogicGateBuilder::new()
                    .add_input("a")
                    .set_output("o_a")
                    .add_truth_table_row((
                        vec![InputValue::Complemented], InputValue::Uncomplemented,
                    )).build().unwrap())
            .add_logic_gate(
                LogicGateBuilder::new()
                    .add_input("b")
                    .set_output("o_b")
                    .add_truth_table_row((
                        vec![InputValue::Complemented], InputValue::Uncomplemented,
                    )).build().unwrap()
            ).build();

        let res = not2.stim(
            SignalsBuilder::new()
                .add_signal("a", SignalState::Low)
                .add_signal("b", SignalState::High)
                .build()
        );

        assert_eq!(res.get("o_a"), SignalState::High);
        assert_eq!(res.get("o_b"), SignalState::Low);
    }
}
