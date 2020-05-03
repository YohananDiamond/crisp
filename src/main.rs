extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod lib;

fn main() {
    let code = r#"(println "Hello, World!") (println "This should be on another line.") (println (+ 1 2))"#;
    let exit_code = init_interpreter(code);
    std::process::exit(exit_code);
}

fn init_interpreter(code: &str) -> i32 {
    use lib::data::Data;
    use lib::interpreter::Interpreter;
    println!("Code: {}", code);

    match lib::parser::parse_program(code) {
        Ok(prog) => {
            let mut interpreter =
                Interpreter::new(prog.iter().map(|pre| Data::from(pre.clone())).collect());
            interpreter.start()
        }
        Err(e) => {
            println!("Parsing error:\n{}", e);
            2
        }
    }
}
