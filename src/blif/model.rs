use super::LogicGate;
use crate::simulation::Simulable;

use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
pub struct Model {
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,

    pub gates: Vec<LogicGate>,
}

impl Model {
    pub fn new(name: String, inputs: Vec<String>, outputs: Vec<String>, gates: Vec<LogicGate>) -> Self {
        Self { name, inputs, outputs, gates }
    }
}

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
    use crate::blif::*;
    use crate::simulation::*;

    #[test]
    fn test_model_not2() {
        let not2 = Model::new(
            "not2".into(),
            vec!["a".into(), "b".into()],
            vec!["o_a".into(), "o_b".into()],
            vec![
                LogicGate::new(
                    vec!["a".into()],
                    "o_a".into(),
                    vec![
                        (vec![InputValue::Complemented], InputValue::Uncomplemented)
                    ]
                ),
                LogicGate::new(
                    vec!["b".into()],
                    "o_b".into(),
                    vec![
                        (vec![InputValue::Complemented], InputValue::Uncomplemented)
                    ]
                ),
            ]
        );

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
