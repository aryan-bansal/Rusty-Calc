# Rusty Calc

**Rusty Calc** is a lightweight, efficient command-line calculator built using Rust. It supports basic arithmetic operations and implements a custom lexer and parser to tokenize and evaluate mathematical expressions using the Shunting Yard algorithm. This project is optimized for memory management, error handling, and scalability.


## Features

- **Basic Arithmetic**: Supports addition, subtraction, multiplication, and division.
- **Variables Support**: Includes predefined constants like `pi` and `e`, and allows custom variables.
- **Error Handling**: Graceful error handling for invalid input and unknown variables.


## Installation

To clone the repository and run Rusty Calc on your local machine:

```bash
git clone https://github.com/yourusername/Rusty-Calc.git
cd Rusty-Calc
cargo build
cargo run
```


## Usage

After compiling and running the program, you can input mathematical expressions for evaluation. Here are a few examples:

```plaintext
Input: 3 + 4 * 2
Output: 11

Input: e * 2 + pi
Output: 8.57914628
```


## Contributing

Feel free to fork the project and submit pull requests. Ensure your changes pass all tests and maintain the structure of the project.
