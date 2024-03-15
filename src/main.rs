use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub struct Variable<'a> {
    pub name: &'a str,
}

fn parse_variable(input: &str) -> Option<(Variable, &str)> {
    if let Some(start_index) = input.find("{{") {
        if let Some(end_index) = input[start_index + 2..].find("}}") {
            let name = &input[start_index + 2..start_index + 2 + end_index];
            let remaining = &input[start_index + 2 + end_index + 2..];
            let variable = Variable { name };
            return Some((variable, remaining));
        }
    }
    None
}

fn transpile(template: &str, context: &HashMap<&str, &str>) -> String {
    let mut output = String::new();
    let mut remaining = template;

    while let Some((variable, new_remaining)) = parse_variable(remaining) {
        if let Some(value) = context.get(variable.name) {
            output.push_str(value);
        } else {
            output.push_str(&format!("{{{{ {} }}}}", variable.name));
        }
        remaining = new_remaining;
    }

    output.push_str(remaining);
    output
}

fn main() {
    // let input = "Hello, {{ name }}! Your score is {{ score }}.";

    // if let Some((variable1, remaining)) = parse_variable(input) {
    //     println!("Variable: {:?}", variable1);

    //     if let Some((variable2, remaining)) = parse_variable(remaining) {
    //         println!("Variable: {:?}", variable2);
    //         println!("Remaining: {}", remaining);
    //     }
    // }

    let template = "<p> {{name}} </p>";
    let context: HashMap<&str, &str> = [("name", "Regis")].iter().cloned().collect();
    let output = transpile(template, &context);
    println!("{}", output);
}
