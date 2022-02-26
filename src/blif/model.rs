use super::LogicGate;

#[derive(PartialEq, Debug)]
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
