#[derive(Debug)]
pub enum OperatorType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Operator(OperatorType),
    Number(f64),
    Variable(String),
}
