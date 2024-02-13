use crate::token_module::{OperatorType, Token};

pub struct Lexer {
    input_chars: Vec<char>,
    current_index: usize,
}

impl Lexer {
    pub fn new(input_string: String) -> Self {
        Self {
            input_chars: input_string.trim().chars().collect(),
            current_index: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut token_list = Vec::new();

        while !self.is_at_end() {
            match self.peek_char() {
                '0'..='9' => token_list.push(self.parse_number()?),
                'a'..='z' | 'A'..='Z' => token_list.push(self.parse_variable()?),
                '+' => {
                    token_list.push(Token::Operator(OperatorType::Add));
                    self.advance_char();
                }
                '-' => {
                    token_list.push(Token::Operator(OperatorType::Subtract));
                    self.advance_char();
                }
                '*' => {
                    token_list.push(Token::Operator(OperatorType::Multiply));
                    self.advance_char();
                }
                '/' => {
                    token_list.push(Token::Operator(OperatorType::Divide));
                    self.advance_char();
                }
                ' ' | '\n' | '\t' => {
                    self.advance_char();
                }
                _ => return Err(format!("Invalid character '{}' found in input", self.peek_char())),
            }
        }

        Ok(token_list)
    }

    fn peek_char(&self) -> char {
        self.input_chars[self.current_index]
    }

    fn advance_char(&mut self) {
        self.current_index += 1;
    }

    fn is_at_end(&self) -> bool {
        self.current_index >= self.input_chars.len()
    }

    fn parse_number(&mut self) -> Result<Token, String> {
        let start_index = self.current_index;

        while !self.is_at_end() && self.peek_char().is_digit(10) {
            self.advance_char();
        }

        if !self.is_at_end() && self.peek_char() == '.' {
            self.advance_char();
            while !self.is_at_end() && self.peek_char().is_digit(10) {
                self.advance_char();
            }
        }

        let num_string: String = self.input_chars[start_index..self.current_index].iter().collect();
        match num_string.parse::<f64>() {
            Ok(num_value) => Ok(Token::Number(num_value)),
            Err(_) => Err(format!("Failed to parse number: {}", num_string)),
        }
    }

    fn parse_variable(&mut self) -> Result<Token, String> {
        let start_index = self.current_index;

        while !self.is_at_end() && self.peek_char().is_alphabetic() {
            self.advance_char();
        }

        let variable_string: String = self.input_chars[start_index..self.current_index].iter().collect();
        Ok(Token::Variable(variable_string))
    }
}
