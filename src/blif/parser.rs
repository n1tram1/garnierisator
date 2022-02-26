use super::logic_gate::{InputValue, LogicGate};
use super::Model;
use super::Blif;


use nom::{
    IResult,
    error::{
        VerboseError,
        context,
    },
    sequence::{
        preceded,
        delimited,
        terminated,
        tuple,
        pair,
    },
    multi::{
        many0,
        many1,
    },
    bytes::complete::{
        take_while1,
    },
    character::is_alphanumeric,
    character::complete::{
        space0,
        space1,
        one_of,
        char,
    },
    combinator::opt,
    bytes::complete::tag,
};

fn is_valid_name_char(c: char) -> bool {
    is_alphanumeric(c as u8) || c == '_' || c == '.' || c == '$' || c == '/' || c == ':'
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

fn parse_names(input: &str) -> IResult<&str, (Vec<String>, String), VerboseError<&str>> {
    context(
        "names",
        preceded(
            tuple((tag(".names"), space1)),
            many1(parse_name)
        )
    )(input)
        .map(|(next_input, mut res)| {

            let output = res.pop().unwrap();
            let inputs = res;

            (next_input, (inputs, output))
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

fn parse_logic_gate(input: &str) -> IResult<&str, LogicGate, VerboseError<&str>> {
    context(
        "logic-gate",
        pair(
            context("logic-gate-names",
                terminated(parse_names, opt(char('\n')))
            ),
            many0(
                terminated(parse_single_output_cover, char('\n'))
            ),
        )
    )(input)
        .map(|(next_input, (names, single_output_cover))| {
            let (input_names, output_name) = names;

            let gate = LogicGate::new(input_names, output_name, single_output_cover);
            (next_input, gate)
        })
}

fn parse_model_name(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        "model-name",
        delimited(
            terminated(tag(".model"), space1),
            parse_name, char('\n'))
    )(input)
}

fn parse_decl_list(input: &str) -> IResult<&str, Vec<String>, VerboseError<&str>> {
    context(
        "decl-list",
        terminated(
            many1(parse_name),
            char('\n')
        )
    )(input)
}

fn parse_model_inputs(input: &str) -> IResult<&str, Vec<String>, VerboseError<&str>> {
    context(
        "model-name",
        preceded(
            terminated(tag(".inputs"), space1),
            parse_decl_list)
    )(input)
}

fn parse_model_outputs(input: &str) -> IResult<&str, Vec<String>, VerboseError<&str>> {
    context(
        "model-name",
        preceded(
            terminated(tag(".outputs"), space1),
            parse_decl_list
        )
    )(input)
}

fn parse_model(input: &str) -> IResult<&str, Model, VerboseError<&str>> {
    context(
        "model",
        terminated(
            tuple((
                parse_model_name,
                parse_model_inputs,
                parse_model_outputs,
                many0(parse_logic_gate),
            )),
            terminated(tag(".end"), char('\n'))
        )
    )(input)
        .map(|(next_input, (name, inputs, outputs, gates))| {
            (next_input, Model {
                    name,
                    inputs,
                    outputs,
                    gates,
            })
        })
}

fn parse_blif(input: &str) -> IResult<&str, Blif, VerboseError<&str>> {
    context(
        "blif",
        many1(parse_model)
    )(input)
        .map(|(next_input, models)| {
            (next_input, Blif::new(models))
        })
}

pub fn parse(input: &str) -> Blif {
    let (_, blif) = parse_blif(input).unwrap();

    blif
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let names = parse_names(".names $undef.tmp0");

        let expected = (vec![], String::from("$undef.tmp0"));

        assert_eq!(names, Ok(("", expected)));
    }

    #[test]
    fn test_parse_name_invalid_no_names() {
        let names = parse_names(".names");

        assert!(names.is_err());
    }

    #[test]
    fn test_parse_names_two_elements() {
        let names = parse_names(".names i_B Y");

        let expected = (vec![String::from("i_B")], String::from("Y"));

        assert_eq!(names, Ok(("", expected)));
    }

    #[test]
    fn test_parse_names_three_elements_weird_name() {
        let names = parse_names(".names i_B Y$tmp.1 $tmp.8");

        let expected = (
            vec![String::from("i_B"), String::from("Y$tmp.1")],
            String::from("$tmp.8")
        );

        assert_eq!(names, Ok(("", expected)));
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

    #[test]
    fn test_parse_logic_gate_just_names() {
        let logic_gate = parse_logic_gate(".names in1 in2 out1");

        let expected = LogicGate::new(
            vec!["in1".into(), "in2".into()],
            "out1".into(),
            Vec::new(),
        );

        assert_eq!(logic_gate, Ok(("", expected)));
    }

    #[test]
    fn test_parse_logic_gate_empty_names() {
        let logic_gate = parse_logic_gate(".names ");

        assert!(logic_gate.is_err());
    }

    #[test]
    fn test_parse_logic_gate() {
        let logic_gate = parse_logic_gate(".names a b o\n0- 1\n1- 1\n-- 1\n01 1\n");

        let expected = LogicGate::new(
            vec!["a".into(), "b".into()],
            "o".into(),
            vec![
                (vec![InputValue::Complemented, InputValue::NotUsed], InputValue::Uncomplemented),
                (vec![InputValue::Uncomplemented, InputValue::NotUsed], InputValue::Uncomplemented),
                (vec![InputValue::NotUsed, InputValue::NotUsed], InputValue::Uncomplemented),
                (vec![InputValue::Complemented, InputValue::Uncomplemented], InputValue::Uncomplemented),
            ],
        );

        assert_eq!(logic_gate, Ok(("", expected)));
    }

    #[test]
    fn test_parse_model_inputs_outputs() {
        let model = parse_model(concat!(
            ".model test\n",
            ".inputs a b\n",
            ".outputs o\n",
            ".end\n",
        ));

        let expected = Model::new(
            "test".into(),
            vec!["a".into(), "b".into()],
            vec!["o".into()],
            Vec::new(),
        );

        assert_eq!(model, Ok(("", expected)));
    }

    #[test]
    fn test_parse_model() {
        let model = parse_model(concat!(
            ".model test\n",
            ".inputs a b\n",
            ".outputs o\n",
            ".names a b o\n",
            "11 1\n",
            ".end\n",
        ));

        let expected = Model {
            name: "test".into(),
            inputs: vec!["a".into(), "b".into()],
            outputs: vec!["o".into()],
            gates: vec![
                LogicGate { inputs: vec!["a".into(), "b".into()], output: "o".into(), single_output_cover: vec![
                    (vec![InputValue::Uncomplemented, InputValue::Uncomplemented], InputValue::Uncomplemented),
                ]},
            ],
        };

        assert_eq!(model, Ok(("", expected)));
    }
}
