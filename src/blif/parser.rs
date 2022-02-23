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
        pair,
    },
    multi::{
        many1,
        many0,
    },
    character::complete::{
        anychar,
        space0,
        alphanumeric1,
        char,
        alpha1,
        space1,
        digit1,
        one_of,
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
    combinator::opt,
};

pub fn parse(_input: &str) -> Blif {
    let lut1 = LogicGateBuilder::new()
        .add_input("i_A")
        .add_input("Y")
        .set_output("o_led")
        .add_truth_value(vec![InputValue::Uncomplemented, InputValue::Uncomplemented])
        .build().unwrap();

    let lut2 = LogicGateBuilder::new()
        .add_input("i_B")
        .set_output("Y")
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
        preceded(
            tuple((tag(".names"), space1)),
            many1(parse_name)
        )
    )(input)
        .map(|(next_input, res)| {

            builder = res[0..res.len()-1].iter().fold(builder, |b, input| b.add_input(input));
            builder = builder.set_output(&res.last().unwrap());

            (next_input, builder.build())
        })
}

fn parse_single_output_cover(input: &str) -> IResult<&str, (Vec<InputValue>, InputValue), VerboseError<&str>> {
    context(
        "single-output-cover",
        pair(
            opt(
                terminated(
                    many0(one_of("01-")),
                    space1
                )
            ),
            one_of("01")
        )
    )(input)
        .map(|(next_input, res)| {
            let (maybe_inputs, output) = res;

            let inputs = maybe_inputs.map_or(vec![], |inputs| {
                inputs
                    .iter()
                    .map(|x| InputValue::try_from(x).unwrap())
                    .collect()
            });
            let output = InputValue::try_from(output).unwrap();

            (next_input, (inputs, output))
        })
}

#[cfg(test)]
mod tests {
    use super::parser::{parse_name, parse_names, parse_single_output_cover};
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
    fn test_parse_names_one_weird_name() {
        let logic_gate = parse_names(".names $undef.tmp0");


        let mut expected = LogicGateBuilder::new()
            .set_output("$undef.tmp0")
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
            .add_input("i_B")
            .set_output("Y")
            .build();

        assert_eq!(logic_gate, Ok(("", expected)));
    }

    #[test]
    fn test_parse_names_three_elements_weird_name() {
        let logic_gate = parse_names(".names i_B Y$tmp.1 $tmp.8");


        let mut expected = LogicGateBuilder::new()
            .add_input("i_B")
            .add_input("Y$tmp.1")
            .set_output("$tmp.8")
            .build();

        assert_eq!(logic_gate, Ok(("", expected)));
    }

    #[test]
    fn test_parse_single_output_cover_one_output_1() {
        let single_output_cover = parse_single_output_cover("1");

        assert_eq!(single_output_cover, Ok(("", (vec![], InputValue::Uncomplemented))));
    }

    #[test]
    fn test_parse_single_output_cover_one_output_0() {
        let single_output_cover = parse_single_output_cover("0");

        assert_eq!(single_output_cover, Ok(("", (vec![], InputValue::Complemented))));
    }

    #[test]
    fn test_parse_single_output_cover_one_input_one_output_01() {
        let single_output_cover = parse_single_output_cover("0 1");

        assert_eq!(single_output_cover, Ok(("", (vec![InputValue::Complemented], InputValue::Uncomplemented))));
    }

    #[test]
    fn test_parse_single_output_cover_one_input_one_output_11() {
        let single_output_cover = parse_single_output_cover("1 1");

        assert_eq!(single_output_cover, Ok(("", (vec![InputValue::Uncomplemented], InputValue::Uncomplemented))));
    }

    #[test]
    fn test_parse_single_output_cover_multiple_inputs_one_output() {
        let single_output_cover = parse_single_output_cover("1-01 1");

        let inputs = vec![
            InputValue::Uncomplemented,
            InputValue::NotUsed,
            InputValue::Complemented,
            InputValue::Uncomplemented,
        ];

        assert_eq!(single_output_cover, Ok(("", (inputs, InputValue::Uncomplemented))));
    }
}
