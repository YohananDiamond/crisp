mod lib;
use lib::{
    parser::parse_tokens,
    lexer::Lexer,
    interpreter::{Interpreter, Data},
};

fn main() {
    let code = r"(hello-world) (hello-world) (hello-world) (+ 1 2)";
    let exit_code = init_interpreter(code);
    std::process::exit(exit_code);
}

fn init_interpreter(code: &str) -> i32 {
    println!("Code: {}", code);

    let tokens = Lexer::from(code).get_tokens();
    match tokens {
        Ok(t) => match parse_tokens(&t) {
            Ok(expr) => {
                let data: Vec<Data> = expr.iter().map(|x| Data::from(x.clone())).collect();
                let mut interpreter = Interpreter::new(data);
                interpreter.start() // TODO: choose between run data and/or REPL
            },
            Err(e) => {
                eprintln!("Could not parse input: {:?}", e); // TODO: print a better stack trace and etc.
                1
            },
        },
        Err(e) => {
            eprintln!("Could not tokenize input: {:?}", e);
            1
        },
    }
}
