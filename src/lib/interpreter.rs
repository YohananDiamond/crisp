use crate::lib::data::Data;
use std::collections::HashMap;

type Scope<'a> = HashMap<&'a str, Data>;
pub type EvalResult = Result<Data, String>; // A method for raising exceptions. I should make something better later.

// TODO: Make a way to choose between REPL & Run File (also, do command-line parsing)
pub struct Interpreter<'a> {
    scopes: Vec<Scope<'a>>,
    program: Vec<Data>,
}

impl<'a> Interpreter<'a> {
    pub fn new(data: Vec<Data>) -> Interpreter<'a> {
        let mut interpreter = Interpreter {
            scopes: Vec::new(),
            program: data,
        };
        interpreter
            .scopes
            .push(Interpreter::make_standard_library());

        interpreter
    }

    fn make_standard_library() -> Scope<'a> {
        let mut standard: Scope = HashMap::new();

        standard.insert("print", Data::RustFunction(|args| {
            let mut output_base = Vec::<String>::new();
            for arg in args {
                output_base.push(arg.to_lisp_string());
            }
            print!("{}", output_base.join(" "));
            Ok(Data::Nil)
        }));

        standard.insert("println", Data::RustFunction(|args| {
            let mut output_base = Vec::<String>::new();
            for arg in args {
                output_base.push(arg.to_lisp_string());
            }
            println!("{}", output_base.join(" "));
            Ok(Data::Nil)
        })); // TODO: find a better way to stop repeating code here

        standard.insert("+", Data::RustFunction(|args| {
            let mut current = Data::Int(0);
            for arg in args {
                match arg {
                    Data::Int(a) => match current {
                        Data::Float(x) => current = Data::Float(x + (*a as f64)),
                        Data::Int(x) => current = Data::Int(x + a),
                        _ => unreachable!(),
                    },
                    Data::Float(a) => match current {
                        Data::Float(x) => current = Data::Float(x + a),
                        Data::Int(x) => current = Data::Float((x as f64) + a),
                        _ => unreachable!(),
                    },
                    _ => return Err(format!("attempted to use {:?} in function + (wrong argument type)", arg)),
                }
            }

            Ok(current)
        }));

        standard
    }

    pub fn start(&mut self) -> i32 {
        for data in self.program.clone() {
            if let Err(e) = self.eval(Ok(data)) {
                /* Terrible debug here, huh? */
                println!("An error ocurred: {}", e);
                return 1;
            }
        }
        0
    }

    pub fn eval(&mut self, data: EvalResult) -> EvalResult {
        match data? {
            Data::List(list) => {
                // Unquoted list, A.K.A. function call
                if list.len() > 0 {
                    let mut evaluated_list = list.clone();
                    for item in evaluated_list.iter_mut() {
                        *item = match item {
                            Data::List(list) => self.eval(Ok(item.clone())), // Function recursion
                            Data::Symbol(symbol) => match self.scope_lookup(&symbol) {
                                // Try to look up variable
                                Some(thing) => Ok(thing),
                                None => Err(format!(r#"Could not find variable "{}""#, symbol)),
                            },
                            _ => Ok(item.clone()),
                        }?;
                    }

                    match &evaluated_list[0] {
                        // Panic-safe because it's been asserted before that the list length is greater than 0
                        Data::RustFunction(f) => {
                            if list.len() == 1 {
                                f(&[])
                            } else {
                                f(&list[1..])
                            }
                        },
                        x => Err(format!("Is not a function: {}", x.repr())),
                    }
                } else {
                    Ok(Data::Nil)
                }
            }
            any => Ok(any),
        }
    }

    fn scope_lookup(&self, var_name: &'a str) -> Option<Data> {
        for (_scope_index, scope) in self.scopes.iter().enumerate().rev() {
            if let Some(data) = scope.get(var_name) {
                return Some(data.clone());
            }
        }

        None
    }
}
