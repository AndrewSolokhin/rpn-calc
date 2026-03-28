pub mod types;
use crate::types::{RpnElements, RpnError, RpnOperator, StackMemory};

fn processing_data(input: &str) -> Result<Vec<String>, RpnError> {
    let mut stack: Vec<String> = Vec::new();

    for element in input.split_whitespace() {
        stack.push(element.to_string());
    }

    if stack.is_empty() {
        return Err(RpnError::ParseError);
    }
    Ok(stack)
}

fn processing_elements(input: &str) -> Result<Vec<RpnElements>, RpnError> {
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

fn calculate(input: Vec<RpnElements>) -> Result<StackMemory, RpnError> {
    let mut stack: Vec<f64> = Vec::new();

    for element in input {
        match element {
            RpnElements::Value(i) => stack.push(i),
            RpnElements::Operator(op) => {
                let right = stack.pop().ok_or(RpnError::MissingOperand)?;
                let left = stack.pop().ok_or(RpnError::MissingOperand)?;

                match op {
                    RpnOperator::Add => stack.push(left + right),
                    RpnOperator::Sub => stack.push(left - right),
                    RpnOperator::Mul => stack.push(left * right),
                    RpnOperator::Div => {
                        if right == 0.0 {
                            return Err(RpnError::DivisionByZero);
                        }
                        stack.push(left / right);
                    }
                }
            }
        }
    }

    if stack.len() != 1 {
        Err(RpnError::ParseError)
    } else {
        Ok(StackMemory { memory: stack })
    }
}
