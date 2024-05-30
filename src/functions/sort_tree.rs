use std::path::Display;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, one_of},
    sequence::{pair, preceded},
    IResult,
};

enum DisplayOrder {
    Ascend,
    Descend,
}

enum CommandType {
    Insert(i32),
    Display(DisplayOrder),
}

#[derive(PartialEq)]
struct ValueNode {
    value: i32,
    left_node: Option<Box<ValueNode>>,
    right_node: Option<Box<ValueNode>>,
}

impl ValueNode {
    fn new(value: i32) -> Self {
        Self {
            value: value,
            left_node: None,
            right_node: None,
        }
    }
}

struct SortTree {
    root: Option<Box<ValueNode>>,
}

impl Default for SortTree {
    fn default() -> Self {
        Self { root: None }
    }
}

impl SortTree {
    fn insert(self, value: i32) {}
    fn display(self, order: DisplayOrder) {}
}

pub fn run_cli(command: String) {
    if let Ok((_, c)) = parse_cli(&command) {
        match c {
            CommandType::Insert(value) => {
                println!("Insert Value: {value}");
            }
            CommandType::Display(DisplayOrder::Ascend) => {
                println!("Ascend Sort");
            }
            CommandType::Display(DisplayOrder::Descend) => {
                println!("Descend Sort");
            }
        }
    } else {
        println!("Unknown command!");
    }
}

fn parse_cli(command: &str) -> IResult<&str, CommandType> {
    alt((is_cli_insert, is_cli_display))(command)
}

fn ws(input: &str) -> IResult<&str, char> {
    one_of(" \t")(input)
}

fn is_char(c: char) -> bool {
    c.is_alphabetic() || c.is_whitespace()
}

fn is_cli_insert(command: &str) -> IResult<&str, CommandType> {
    match preceded(pair(tag("insert"), ws), digit1)(command) {
        Err(err) => Err(err),
        Ok((rest, value)) => Ok((
            rest,
            CommandType::Insert(value.trim().parse::<i32>().unwrap()),
        )),
    }
}

fn is_cli_display(command: &str) -> IResult<&str, CommandType> {
    if command == "display" {
        return Ok(("display", CommandType::Display(DisplayOrder::Ascend)));
    }
    match preceded(pair(tag("display"), ws), take_while1(is_char))(command) {
        Err(e) => Err(e),
        Ok((rest, wot)) => {
            if wot == "ascend" {
                Ok((rest, CommandType::Display(DisplayOrder::Ascend)))
            } else if wot == "descend" {
                Ok((rest, CommandType::Display(DisplayOrder::Descend)))
            } else {
                Ok((rest, CommandType::Display(DisplayOrder::Ascend)))
            }
        }
    }
}
