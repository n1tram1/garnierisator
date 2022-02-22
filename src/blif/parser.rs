use super::*;

use nom::{
    IResult,
    error::{
        VerboseError,
        context,
    },
    sequence::{
        terminated,
        tuple,
        preceded,
    },
    multi::many1,
    character::complete::{
        anychar,
        space0,
        alphanumeric1,
        char,
        alpha1,
        space1,
        digit1,
    },
    character::{
        is_alphanumeric,
        is_hex_digit,
    },
    bytes::complete::{tag, take_while1},
    branch::{
        alt,
        permutation,
    },
};

pub fn parse(_input: &str) -> Blif {
    let lut1 = LogicGateBuilder::new()
        .add_io("i_A")
        .add_io("Y")
        .add_io("o_led")
        .add_truth_value(vec![InputValue::Uncomplemented, InputValue::Uncomplemented])
        .build().unwrap();

    let lut2 = LogicGateBuilder::new()
        .add_io("i_B")
        .add_io("Y")
        .add_truth_value(vec![InputValue::Complemented])
        .build().unwrap();

    Blif {
        models: vec![
            Model {
                name: String::from("blinky"),
                inputs: vec![String::from("i_A"), String::from("i_B")],
                outputs: vec![String::from("o_led")],

                gates: vec![
                    lut1, lut2,
                ],
            },
        ],
    }
}

fn is_valid_name_char(c: char) -> bool {
    is_alphanumeric(c as u8) || c == '_' || c == '.' || c == '$'
}

fn parse_name(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        "name",
        terminated(
            take_while1(|c| is_valid_name_char(c)),
            space0
        )
    )(input)
        .map(|(next_input, res)| (next_input, res.into()))
}

fn parse_names(input: &str) -> IResult<&str, Result<LogicGate, &'static str>, VerboseError<&str>> {
    let mut builder = LogicGateBuilder::new();

    context(
        "names",
        preceded(tuple((tag(".names"), space1)), many1(parse_name))
    )(input)
        .map(|(next_input, res)| {
            let mut builder = res.iter().fold(builder, |b, io| b.add_io(io));

            (next_input, builder.build())
        })
}

#[cfg(test)]
mod tests {
    use super::parser::{parse_name, parse_names};
    use super::{LogicGateBuilder, InputValue};

    #[test]
    fn test_parse_name_simple() {
        let name = parse_name("i_B");

        assert_eq!(name, Ok(("", "i_B".into())));
    }

    #[test]
    fn test_parse_name_whitespace_before() {
        let name = parse_name("  i_B");

        assert!(name.is_err());
    }

    #[test]
    fn test_parse_name_whitespace_after() {
        let name = parse_name("i_B  ");

        assert_eq!(name, Ok(("", "i_B".into())));
    }

    #[test]
    fn test_parse_one_weird_name() {
        let logic_gate = parse_names(".names $undef.tmp0");


        let mut expected = LogicGateBuilder::new()
            .add_io("$undef.tmp0")
            .build();

        assert_eq!(logic_gate, Ok(("", expected)));
    }

    #[test]
    fn test_parse_name_invalid_no_names() {
        let logic_gate = parse_names(".names");

        assert!(logic_gate.is_err());
    }

    #[test]
    fn test_parse_names_two_elements() {
        let logic_gate = parse_names(".names i_B Y");


        let mut expected = LogicGateBuilder::new()
            .add_io("i_B")
            .add_io("Y")
            .build();

        assert_eq!(logic_gate, Ok(("", expected)));
    }

    #[test]
    fn test_parse_names_three_elements_weird_name() {
        let logic_gate = parse_names(".names i_B Y$tmp.1 $tmp.8");


        let mut expected = LogicGateBuilder::new()
            .add_io("i_B")
            .add_io("Y$tmp.1")
            .add_io("$tmp.8")
            .build();

        assert_eq!(logic_gate, Ok(("", expected)));
    }
}
