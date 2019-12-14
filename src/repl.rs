/// REPL: Read, Evaluate, Print, Loop

use std::io;
use std::io::Write;
use crate::parser;

const PROMPT: &str = "repl> ";

/**
 * Starts the REPL.
 */
pub fn start() {
    loop {

        // Get the input
        let input = get_input();

        // Print back what was typed, but in debug mode.
        // This was supposed to be where the input is processed into tokens, but it is not done yet.
        // println!("{:?}", input);
        parser::parse_string(&input);

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
    io::stdout().flush().unwrap(); // @todo: Search why is this needed.

    // Actually read input
    io::stdin().read_line(&mut input)
        .expect("Error reading input");

    // Return the trimmed String
    input.trim().to_string()
}
