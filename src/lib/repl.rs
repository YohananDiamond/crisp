/// A REPL (or Read, Evaluate, Print, Loop) is a program that can get
/// input from the user, process that in a certain language, and return
/// the results to the user.

use std::io::{self, Write};
use crate::lib::{
    lexer::Lexer,
    parser::{parse_tokens, ParserResult, Expression},
};

const PROMPT: &str = "repl> ";
const INDENT_SIZE: usize = 2;

/**
 * Initializes the REPL and processes the user the user input until it
 * SIGINT (^C) is sent to the terminal.
 */
pub fn init() {
    loop {
        // Get the input
        let input = get_input();

        // Prints back the input
        let mut lex = Lexer::from(&input);
        let tokens = lex.get_tokens();
        match tokens {
            Ok(tok) => print_expression_tree(&parse_tokens(&tok)),
            Err(e) => println!("Lexer error: {:?}", e),
        }
    }
}

fn print_expression_tree(tree: &ParserResult) {
    match tree {
        Ok(t) => for item in t {
            print_expression(item, 0);
        },
        Err(e) => println!("Parser error: {:?}", e),
    }
}

fn print_expression(e: &Expression, indent: usize) {
    match e {
        Expression::List(l) => {
            println!("{}* List{}", " ".repeat(INDENT_SIZE * indent), if l.len() == 0 { " [empty]" } else { ":" });
            for expression in l {
                print_expression(expression, indent + 1);
            }
        },
        _ => println!("{}* {:?}", " ".repeat(INDENT_SIZE * indent), e),
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
