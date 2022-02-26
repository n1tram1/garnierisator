use crate::simulation::{Simulable, SignalState, Signal, Signals};

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

type InputVariables = Vec<InputValue>;

#[derive(Debug, PartialEq, Clone)]
pub struct LogicGate {
    pub inputs: Vec<String>,
    pub output: String,
    pub single_output_cover: Vec<(InputVariables, InputValue)>,
}

impl LogicGate {
    pub fn new(inputs: Vec<String>, output: String, single_output_cover: Vec<(InputVariables, InputValue)>) -> Self {
        Self { inputs, output, single_output_cover }
    }
}

fn apply(input_values: &Vec<InputValue>, signals: &Vec<SignalState>) -> bool {
    for (i, s) in std::iter::zip(input_values, signals) {
        if i == &InputValue::Complemented && s == &SignalState::High ||
            i == &InputValue::Uncomplemented && s == &SignalState::Low {
            return false;
        }
    }

    return true;
}

use std::collections::HashSet;

impl Simulable for LogicGate {
    fn get_inputs(&self) -> HashSet<String> {
        self.inputs.clone().into_iter().collect()
    }

    fn children(&self) -> Vec<Box<dyn Simulable>> {
        unimplemented!();
    }

    fn stim(&self, signals: Signals) -> Signals {
        let bound_signals = self.inputs.iter().fold(Vec::new(), |mut bound, input_name| {
            let value = signals.get(input_name);

            bound.push(value);

            bound
        });

        let mut output = Signal::new(&self.output);
        output.set_low();

        for (row_inputs, row_output) in &self.single_output_cover {
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
    use super::*;
    use crate::simulation::*;

    lazy_static::lazy_static! {
        static ref AND_GATE: LogicGate = LogicGate::new(
            vec!["a".into(), "b".into()],
            "y".into(),
            vec![
                (vec![InputValue::Uncomplemented, InputValue::Uncomplemented], InputValue::Uncomplemented)
            ]);

        static ref NOT_GATE: LogicGate = LogicGate::new(
            vec!["a".into()],
            "y".into(),
            vec![
                (vec![InputValue::Complemented], InputValue::Uncomplemented),
            ]
        );
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
