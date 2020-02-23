/// A REPL (or Read, Evaluate, Print, Loop) is a program that can get
/// input from the user, process that in a certain language, and return
/// the results to the user.

use std::io::{self, Write};
use crate::lib::{
    lexer::Lexer,
    parser::{tokens_to_expression, ExpressionTree},
};

const PROMPT: &str = "repl> ";

/**
 * Initializes the REPL and processes the user the user input until it
 * SIGINT (^C) is sent to the terminal.
 */
pub fn init() {
    loop {
        // Get the input
        let input = get_input();

        // Prints back the input
        let mut lex = Lexer::from(input);
        let tokens = lex.get_tokens();
        match tokens {
            Ok(tok) => print_expression_tree(&tokens_to_expression(&tok)),
            Err(e) => println!("Lexer error: {:?}", e),
        }
    }
}

fn print_expression_tree(tree: &ExpressionTree) {
    match tree {
        Ok(t) => for item in t {
            println!("* {:?}", item);
        },
        Err(e) => println!("Parser error: {:?}", e),
    }
}

/**
 * Gets text input from the user.
 * Leading whitespace characters (including \n) are removed.
 */
fn get_input() -> String {
    let mut input = String::new();

    // Show the prompt
    print!("{}", PROMPT);
    io::stdout().flush().unwrap();

    // Actually read input
    io::stdin().read_line(&mut input)
        .expect("Error reading input");

    // Return the trimmed String
    input.trim().to_string()
}
