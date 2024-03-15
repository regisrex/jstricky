use nom::{branch::alt, bytes::complete::tag, character::complete::alpha1, sequence::delimited};
use std::{collections::HashMap, error::Error, fs};

#[derive(Debug)]
pub struct IfStatement<'a> {
    pub condition: &'a str,
    pub true_branch: &'a Vec<Node<'a>>,
    pub false_branch: &'a Vec<Node<'a>>,
}

#[derive(Debug)]
pub struct Context {
    data: HashMap<String, String>,
}

#[derive(Debug)]
pub struct ForLoop<'a> {
    pub items: &'a str,
    pub body: Vec<Node<'a>>,
}

#[derive(Debug)]
pub enum Node<'a> {
    Variable(Variable<'a>),
    Text(&'a str),
    IfStatement(Box<IfStatement<'a>>),
    ForLoop(Box<ForLoop<'a>>),
}

// fn parse_text(input: &str) -> nom::IResult<&str, Node> {
//     let (remaining, text) = nom::bytes::complete::is_not("{{{% ")(input)?;
//     Ok((remaining, Node::Text(text)))
// }

// fn parse_template(input: &str) -> nom::IResult<&str, Vec<Node>> {
//     let mut nodes = Vec::new();
//     let mut remaining = input;

//     loop {
//         if remaining.is_empty() {
//             return Ok((remaining, nodes));
//         }

//         let (new_remaining, node) = alt((
//             parse_variable,
//             // parse_if_statement,
//             // parse_for_loop,
//             // ... other parsers
//             parse_text,
//         ))(remaining)?;

//         nodes.push(node);
//         remaining = new_remaining;
//     }
// }

// pub fn parse_template_file(path: &str) /* ->  Result<Vec<Node>, std::io::Error> */
// {
//     let template_str = fs::read_to_string(path);
//     match template_str {
//         Ok(text) => print!("{}", text),
//         Err(err) => panic!(),
//     }

//     // let (remaining, ast) = parse_template(&template_str)
//     //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;

//     // if !remaining.is_empty() {
//     //     return Err(std::io::Error::new(
//     //         std::io::ErrorKind::InvalidData,
//     //         format!("Failed to parse template, remaining input: {}", remaining),
//     //     ));
//     // }
// }
