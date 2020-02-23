/// A REPL (or Read, Evaluate, Print, Loop) is a program that can get
/// input from the user, process that in a certain language, and return
/// the results to the user.

use std::io::{self, Write};
use crate::lib::{
    lexer::{Token, Lexer, LexerError},
    parser::{tokens_to_expression, Expression},
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
        // print_tokenized(&tokens);
        match tokens {
            Ok(t) => println!("{:?}", tokens_to_expression(&t)),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

/**
 * (Incomplete)
 * Reads tokens from a vector and prints them in a "pretty" way.
 */
pub fn print_tokenized(tokens: &Result<Vec<Token>, LexerError>) {
    if let Ok(x) = tokens {
        for token in x {
            println!("* {:?}", token);
        }
    } else if let Err(x) = tokens {
        println!("Error: {:?}", x);
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