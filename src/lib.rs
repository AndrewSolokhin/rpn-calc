use crate::RpnError::ParseError;

#[derive(PartialEq, Debug)]
struct StackMemory {
    memory: Vec<f64>,
}
#[derive(PartialEq, Debug)]
enum RpnOperator {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(PartialEq, Debug)]
enum RpnElements {
    Operator(RpnOperator),
    Value(f64),
}
#[derive(PartialEq, Debug)]
enum RpnError {
    MissingOperand(String),
    ParseError(String),
    DivisionByZero(String),
}

fn processing_data(input: &str) -> Result<Vec<String>, RpnError> {
    let mut stack: Vec<String> = Vec::new();
    for element in input.split_whitespace() {
        stack.push(element.to_string());
    }

    if stack.is_empty() {
        return Err(ParseError(input.to_string()));
    }
    Ok(stack)
}