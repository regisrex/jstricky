// use nom::{branch::alt, bytes::complete::tag, character::complete::alpha1, sequence::delimited, IResult};

// extern crate nom;

// pub enum Node<'a> {
//     Variable(&'a str),
//     Text(&'a str),
//     IfStatement(Box<IfStatement<'a>>),
//     ForLoop(Box<ForLoop<'a>>),
// }


// fn transpile<'a>(node: &'a Node<'a>) -> TranspiledNode<'a> {
//     match node {
//         Node::Text(text) => TranspiledNode::Text(text),
//         Node::Variable(var) => TranspiledNode::Variable(VariablePlaceholder {
//             name: var.name,
//         }),
//         Node::IfStatement(stmt) => {
//             let true_branch = stmt.true_branch.iter().map(transpile).collect();
//             let false_branch = stmt.false_branch.iter().map(transpile).collect();
//             TranspiledNode::Condition(ConditionPlaceholder {
//                 condition: &stmt.condition,
//                 true_branch,
//                 false_branch,
//             })
//         }
//         Node::ForLoop(loop_node) => TranspiledNode::Loop(LoopPlaceholder {
//             items: &loop_node.items,
//             body: loop_node.body.iter().map(transpile).collect(),
//         }),
//     }

//     fn render_transpiled_node<'a>(node: &TranspiledNode<'a>, context: &Context) -> String {
//         match node {
//             TranspiledNode::Text(text) => text.to_string(),
//             TranspiledNode::Variable(placeholder) => context.get(&placeholder.name).unwrap_or(&placeholder.name).to_string(),
//             TranspiledNode::Condition(placeholder) => {
//                 let condition_value = context.get(&placeholder.condition).unwrap_or("false");
//                 let branch = if condition_value == "true" {
//                     &placeholder.true_branch
//                 } else {
//                     &placeholder.false_branch
//                 };
//                 branch.iter().map(|n| render_transpiled_node(n, context)).collect()
//             }
//             TranspiledNode::Loop(placeholder) => {
//                 let items = context.get(&placeholder.items).unwrap_or("");
//                 let mut output = String::new();
//                 for item in items.split(',') {
//                     let mut item_context = context.clone();
//                     item_context.insert("item", item.trim());
//                     for node in &placeholder.body {
//                         output.push_str(&render_transpiled_node(node, &item_context));
//                     }
//                 }
//                 output
//             }
//         }
//     }


// fn parse_text(input: &str) -> nom::IResult<&str, Node> {
//     let (remaining, text) = nom::bytes::complete::is_not("{{{% ")(input)?;
//     Ok((remaining, Node::Text(text)))
// }

// fn parse_variable(input: &str) -> nom::IResult<&str, Node> {
//     let (remaining, name) = delimited(tag("{{"), input, tag("}}"))(input)?;

//     Ok((remaining, Node::Variable(name)))
// }

// fn parse_template(input: &str) -> IResult<&str, Vec<Node>> {
//     let mut nodes = Vec::new();
//     let mut remaining = input;

//     loop {
//         if remaining.is_empty() {
//             return Ok((remaining, nodes));
//         }

//         let (new_remaining, node) = alt((
//             parse_variable,
//             parse_if_statement,
//             parse_for_loop,
//             // ... other parsers
//             parse_text,
//         ))(remaining)?;

//         nodes.push(node);
//         remaining = new_remaining;
//     }
// }
