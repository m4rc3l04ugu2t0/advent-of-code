use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone)]
pub enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

#[derive(PartialEq, Eq)]
enum ShouldProcess {
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

pub fn parse_input_02(input: &str) -> IResult<&str, u32> {
    let (_input, instructions) =
        many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)?;

    let result =
        instructions
            .iter()
            .fold((ShouldProcess::Do, 0), |(process, acc), ins| match ins {
                Instruction::Mul(a, b) => {
                    if process == ShouldProcess::Do {
                        (process, acc + a * b)
                    } else {
                        (process, acc)
                    }
                }
                Instruction::Do => (ShouldProcess::Do, acc),
                Instruction::Dont => (ShouldProcess::Dont, acc),
            });
    Ok(("", result.1))
}

#[test]
fn test_day_3() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))

";

    assert_eq!(parse_input_02(input).unwrap().1, 48);
}
