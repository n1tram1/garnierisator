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

#[derive(Debug, PartialEq)]
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
