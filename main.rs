mod source;

use std::collections::HashMap;
use std::io::stdin;
use source::lexer_module::Lexer;
use source::postfix_module::shunting_yard;
use source::token_module::{OperatorType, Token};


fn main() {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).expect("Failed to read line");

    let mut lexer_instance = Lexer::new(input_string);
    let expression_stream = lexer_instance.tokenize()
        .expect("Failed to tokenize input");

    let postfix_expression = shunting_yard(expression_stream);

    let mut stack_vector = Vec::new();
    let mut variable_map_hash: HashMap<String, f64> = HashMap::new();

    variable_map_hash.insert("pi".to_string(), 3.14159);
    variable_map_hash.insert("e".to_string(), 2.71828);

    for token_instance in postfix_expression {
        match token_instance {
            Token::Number(num) => stack_vector.push(num),
            Token::Variable(variable_string) => {
                let value_instance = variable_map_hash.get(&variable_string)
                    .expect(format!("Variable '{}' not found", variable_string).as_str());
                stack_vector.push(*value_instance);
            }
            Token::Operator(operator_instance) => {
                let right_operand = stack_vector.pop().expect("Failed to pop right operand");
                let left_operand = stack_vector.pop().expect("Failed to pop left operand");

                let result_value = match operator_instance {
                    OperatorType::Add => left_operand + right_operand,
                    OperatorType::Subtract => left_operand - right_operand,
                    OperatorType::Multiply => left_operand * right_operand,
                    OperatorType::Divide => left_operand / right_operand,
                };

                stack_vector.push(result_value);
            }
        }
    }

    println!("Result: {}", stack_vector.pop().expect("Failed to pop result"));
}
