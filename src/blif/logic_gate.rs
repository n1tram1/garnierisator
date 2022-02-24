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

    pub fn build(self) -> Result<LogicGate, &'static str> {
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

use crate::simulation::{Simulable, SignalState, Signal, Signals};

fn apply(input_values: &Vec<InputValue>, signals: &Vec<SignalState>) -> bool {
    for (i, s) in std::iter::zip(input_values, signals) {
        if i == &InputValue::Complemented && s == &SignalState::High ||
            i == &InputValue::Uncomplemented && s == &SignalState::Low {
            return false;
        }
    }

    return true;
}

impl Simulable for LogicGate {
    fn stim(&self, signals: Signals) -> Signals {
        let bound_signals = self.inputs.iter().fold(Vec::new(), |mut bound, input_name| {
            let value = signals.get(input_name);

            bound.push(value);

            bound
        });

        let mut output = Signal::new(&self.output);
        output.set_low();

        for (row_inputs, row_output) in &self.truth_table {
            if apply(&row_inputs, &bound_signals) {
                if row_output == &InputValue::Uncomplemented  {
                    output.set_high();
                }
            }
        }

        let mut outputs = Signals::new();
        outputs.add_signal(output);

        outputs
    }
}

#[cfg(test)]
mod tests {
    use super::{LogicGateBuilder, LogicGate, InputValue};
    use crate::simulation::{Simulable, SignalState, SignalsBuilder};

    lazy_static::lazy_static! {
        static ref AND_GATE: LogicGate = LogicGateBuilder::new()
            .add_input("a")
            .add_input("b")
            .set_output("y")
            .add_truth_table_row((
                vec![InputValue::Uncomplemented, InputValue::Uncomplemented],
                InputValue::Uncomplemented
            )).build().unwrap();

        static ref NOT_GATE: LogicGate = LogicGateBuilder::new()
            .add_input("a")
            .set_output("y")
            .add_truth_table_row((
                vec![InputValue::Complemented], InputValue::Uncomplemented
            )).build().unwrap();
    }

    #[test]
    fn test_not_0() {
        let simulation = NOT_GATE.stim(
            SignalsBuilder::new()
                .add_signal("a", SignalState::Low)
                .build()
        );

        assert_eq!(simulation.get("y"), SignalState::High);
    }

    #[test]
    fn test_not_1() {
        let simulation = NOT_GATE.stim(
            SignalsBuilder::new()
                .add_signal("a", SignalState::High)
                .build()
        );

        assert_eq!(simulation.get("y"), SignalState::Low);
    }

    #[test]
    fn test_and_gate_00() {
        let simulation = AND_GATE.stim(
            SignalsBuilder::new()
                .add_signal("a", SignalState::Low)
                .add_signal("b", SignalState::Low)
                .build()
        );
        assert_eq!(simulation.get("y"), SignalState::Low);
    }

    #[test]
    fn test_and_gate_01() {
        let simulation = AND_GATE.stim(
            SignalsBuilder::new()
                .add_signal("a", SignalState::Low)
                .add_signal("b", SignalState::High)
                .build()
        );
        assert_eq!(simulation.get("y"), SignalState::Low);
    }

    #[test]
    fn test_and_gate_10() {
        let simulation = AND_GATE.stim(
            SignalsBuilder::new()
                .add_signal("a", SignalState::High)
                .add_signal("b", SignalState::Low)
                .build()
        );
        assert_eq!(simulation.get("y"), SignalState::Low);
    }

    #[test]
    fn test_and_gate_11() {
        let simulation = AND_GATE.stim(
            SignalsBuilder::new()
                .add_signal("a", SignalState::High)
                .add_signal("b", SignalState::High)
                .build()
        );
        assert_eq!(simulation.get("y"), SignalState::High);
    }

}
