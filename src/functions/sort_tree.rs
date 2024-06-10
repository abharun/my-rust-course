use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, one_of},
    sequence::{pair, preceded},
    IResult,
};

#[derive(PartialEq)]
enum DisplayOrder {
    Ascend,
    Descend,
}

enum CommandType {
    Insert(i32),
    Display(DisplayOrder),
}

#[derive(PartialEq, Clone, Debug)]
pub struct ValueNode {
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

#[derive(Clone, Debug)]
pub struct SortTree {
    root: Option<Box<ValueNode>>,
}

impl Default for SortTree {
    fn default() -> Self {
        Self { root: None }
    }
}

impl SortTree {
    fn insert(&mut self, value: i32) {
        let mut current = &mut self.root;

        loop {
            match current {
                None => {
                    *current = Some(Box::new(ValueNode::new(value)));
                    break;
                }
                Some(ref mut node) => {
                    if value < node.value {
                        current = &mut node.left_node;
                    } else {
                        current = &mut node.right_node;
                    }
                }
            }
        }
    }

    fn display(node: Option<Box<ValueNode>>, order: DisplayOrder) {
        if node == None {
            return;
        }

        if order == DisplayOrder::Ascend {
            Self::display(node.clone().unwrap().left_node, DisplayOrder::Ascend);
            println!("{:?}", node.clone().unwrap().value);
            Self::display(node.clone().unwrap().right_node, DisplayOrder::Ascend);
        } else {
            Self::display(node.clone().unwrap().right_node, DisplayOrder::Descend);
            println!("{:?}", node.clone().unwrap().value);
            Self::display(node.clone().unwrap().left_node, DisplayOrder::Descend);
        }
    }
}

pub fn run_cli(tree: &mut SortTree, command: String) {
    if let Ok((_, c)) = parse_cli(&command) {
        match c {
            CommandType::Insert(value) => {
                tree.insert(value);
            }
            CommandType::Display(DisplayOrder::Ascend) => {
                SortTree::display(tree.root.clone(), DisplayOrder::Ascend);
            }
            CommandType::Display(DisplayOrder::Descend) => {
                SortTree::display(tree.root.clone(), DisplayOrder::Descend);
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
