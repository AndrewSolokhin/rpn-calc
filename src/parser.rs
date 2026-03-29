use crate::types::{RpnElements, RpnError, RpnOperator};

pub fn processing_data(input: &str) -> Result<Vec<String>, RpnError> {
    let mut stack: Vec<String> = Vec::new();

    for element in input.split_whitespace() {
        stack.push(element.to_string());
    }

    if stack.is_empty() {
        return Err(RpnError::ParseError);
    }
    Ok(stack)
}

pub fn processing_elements(input: &str) -> Result<Vec<RpnElements>, RpnError> {
    let mut stack: Vec<RpnElements> = Vec::new();

    for element in input.split_whitespace() {
        if let Ok(i) = element.parse::<f64>() {
            stack.push(RpnElements::Value(i));
        } else {
            match element {
                "+" => stack.push(RpnElements::Operator(RpnOperator::Add)),
                "-" => stack.push(RpnElements::Operator(RpnOperator::Sub)),
                "*" => stack.push(RpnElements::Operator(RpnOperator::Mul)),
                "/" => stack.push(RpnElements::Operator(RpnOperator::Div)),
                _ => return Err(RpnError::ParseError),
            }
        }
    }

    if stack.is_empty() {
        return Err(RpnError::ParseError);
    }
    Ok(stack)
}
