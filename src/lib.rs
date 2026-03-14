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
