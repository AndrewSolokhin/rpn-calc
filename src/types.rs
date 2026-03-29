#[derive(PartialEq, Debug)]
pub struct StackMemory {
    pub memory: Vec<f64>,
}
#[derive(PartialEq, Debug)]
pub enum RpnOperator {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(PartialEq, Debug)]
pub enum RpnElements {
    Operator(RpnOperator),
    Value(f64),
}
#[derive(PartialEq, Debug)]
pub enum RpnError {
    MissingOperand,
    ParseError,
    DivisionByZero,
    EmptyValue,
}
