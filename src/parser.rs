use crate::types::{RpnElements, RpnError, RpnOperator};

pub fn processing_data(input: &str) -> Result<Vec<&str>, RpnError> {
    let mut stack: Vec<&str> = Vec::new();

    for element in input.split_whitespace() {
        stack.push(element);
    }

    if stack.is_empty() {
        return Err(RpnError::EmptyValue);
    }
    Ok(stack)
}

pub fn processing_elements(input: Vec<&str>) -> Result<Vec<RpnElements>, RpnError> {
    let mut stack: Vec<RpnElements> = Vec::new();

    for element in input.iter() {
        if let Ok(i) = element.parse::<f64>() {
            stack.push(RpnElements::Value(i));
        } else {
            match *element {
                "+" => stack.push(RpnElements::Operator(RpnOperator::Add)),
                "-" => stack.push(RpnElements::Operator(RpnOperator::Sub)),
                "*" => stack.push(RpnElements::Operator(RpnOperator::Mul)),
                "/" => stack.push(RpnElements::Operator(RpnOperator::Div)),
                _ => return Err(RpnError::ParseError),
            }
        }
    }

    if stack.is_empty() {
        return Err(RpnError::EmptyValue);
    }
    Ok(stack)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_processing_data_err_empty() {
        let result = processing_data("");
        assert_eq!(result, Err(RpnError::EmptyValue));
    }
    #[test]
    fn test_processing_elements_err_empty() {
        let result = processing_elements(vec![]);
        assert_eq!(result, Err(RpnError::EmptyValue));
    }
    #[test]
    fn test_processing_elements_err_extra_value() {
        let result = processing_elements(vec!["32", "_", "52", "+"]);
        assert_eq!(result, Err(RpnError::ParseError));
    }
}
