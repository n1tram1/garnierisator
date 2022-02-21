use std::io;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
enum InputValue {
    Uncomplemented,
    Complemented,
    NotUsed,
}

pub trait LogicGate {}

pub struct Lut<const N: usize> {
    names: Vec<String>,
    truth_values: HashSet<[InputValue; N]>,
}

impl<const N: usize> LogicGate for Lut<N> {}

pub struct Model {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,

    gates: Vec<Box<dyn LogicGate>>,
}

pub struct Blif {
    models: Vec<Model>,
}

pub fn parse(_input: &str) -> Blif {
    let mut lut1 = Lut::<2> {
        names: vec![String::from("i_A"), String::from("Y"), String::from("o_led")],
        truth_values: HashSet::new(),
    };
    lut1.truth_values.insert([InputValue::Uncomplemented, InputValue::Uncomplemented]);

    let mut lut2 = Lut::<1> {
        names: vec![String::from("i_B"), String::from("Y")],
        truth_values: HashSet::new(),
    };
    lut2.truth_values.insert([InputValue::Complemented]);

    Blif {
        models: vec![
            Model {
                name: String::from("blinky"),
                inputs: vec![String::from("i_A"), String::from("i_B")],
                outputs: vec![String::from("o_led")],

                gates: vec![
                    Box::new(lut1), Box::new(lut2),
                ],
            },
        ],
    }
}
