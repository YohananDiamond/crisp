mod lib;
use lib::{
    parser::parse_tokens,
    lexer::Lexer,
    interpreter::{Interpreter, Data},
};

fn main() {
    let code: String = String::from("(hello-world)");
    let tokens = Lexer::from(&code).get_tokens();
    match tokens {
        Ok(tok) => {
            match parse_tokens(&tok) {
                Ok(expr) => {
                    let data: Vec<Data> = expr.iter().map(|x| Data::from(x.clone())).collect();
                    let mut interpreter = Interpreter::new(data);
                    interpreter.start();
                },
                Err(e) => println!("Could not parse input; error: {:?}", e),
            }
        },
        Err(e) => println!("Could not tokenize input; error: {:?}", e),
    }
}
