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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::RpnOperator::Div;
    #[test]
    fn test_calculate_div_zero() {
        let result = calculate(vec![
            RpnElements::Value(10.0),
            RpnElements::Value(0.0),
            RpnElements::Operator(Div),
        ]);
        assert_eq!(result, Err(RpnError::DivisionByZero));
    }
    #[test]
    fn test_calculate_div_negative() {
        let result = calculate(vec![
            RpnElements::Value(-91.0),
            RpnElements::Value(20.0),
            RpnElements::Operator(Div),
        ]);
        assert_eq!(
            result,
            Ok(StackMemory {
                memory: vec![-4.55]
            })
        );
    }
}
