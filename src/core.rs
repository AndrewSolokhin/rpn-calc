use crate::types::{RpnElements, RpnError, RpnOperator, StackMemory};

pub fn calculate(input: Vec<RpnElements>) -> Result<StackMemory, RpnError> {
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
