use std::io::{self, Write};
use crate::lexer;

const PROMPT: &str = "repl> ";

/**
 * Starts the REPL.
 */
pub fn start() {
    loop {
        // Get the input
        let input = get_input();

        // Prints back the input
        let mut repl_lexer = lexer::Lexer::with_content(&input);
	let tokens = repl_lexer.get_all_next_tokens();
        println!("Tokens: {:?}", tokens);
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
    io::stdout().flush().unwrap(); // TODO: Search why is this needed.

    // Actually read input
    io::stdin().read_line(&mut input)
        .expect("Error reading input");

    // Return the trimmed String
    input.trim().to_string()
}
