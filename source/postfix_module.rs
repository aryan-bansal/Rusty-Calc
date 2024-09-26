use super::token_module::{OperatorType, Token};

fn get_precedence(operator: &OperatorType) -> u8 {
    match operator {
        OperatorType::Add | OperatorType::Subtract => 1,
        OperatorType::Multiply | OperatorType::Divide => 2,
    }
}

pub fn shunting_yard(expression_tokens: Vec<Token>) -> Vec<Token> {
    let mut output_queue = Vec::new();
    let mut operator_stack = Vec::new();

    for token in expression_tokens {
        match token {
            Token::Number(_) => output_queue.push(token),
            Token::Variable(variable_name) => output_queue.push(Token::Variable(variable_name)),
            Token::Operator(operator) => {
                while let Some(&ref top_token) = operator_stack.last() {
                    if let Token::Operator(top_operator) = top_token {
                        if get_precedence(&operator) <= get_precedence(&top_operator) {
                            output_queue.push(operator_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                operator_stack.push(Token::Operator(operator));
            }
        }
    }

    while let Some(operator) = operator_stack.pop() {
        output_queue.push(operator);
    }

    output_queue
}
