use crate::lib::parser::Expression;
use std::collections::HashMap;
use std::fmt;

type Scope<'a> = HashMap<&'a str, Data>;
type EvalResult = Result<Data, String>; // A method for raising exceptions. I should make something better later.

/**
 * Internal representation of the data.
 * Not sure if it was needed differently from Data, but I made it mostly because of Nil and
 * Function, since they are also data. */
#[derive(Clone)]
pub enum Data {
    Symbol(String),
    Str(String),
    Int(i64),
    Float(f64),
    List(Vec<Data>),
    Function(fn(args: &[Data]) -> EvalResult),
    Nil,
    // TODO: Data::Quoted(Data)
}

impl From<Expression> for Data {
    fn from(expression: Expression) -> Data {
        match expression {
            Expression::Symbol(s) => match s.as_ref() {
                "nil" => Data::Nil,
                _ => Data::Symbol(s),
            },
            Expression::Str(s) => Data::Str(s),
            Expression::Int(i) => Data::Int(i),
            Expression::Float(f) => Data::Float(f),
            Expression::List(v) => Data::List(v.iter().map(|x| Data::from(x.clone())).collect()), // FIXME: maybe optimize to not to use ".clone()" ? I'm not sure what I'm doing here with it.
        }
    }
}

impl Data {
    pub fn repr(&self) -> String {
        match self {
            Data::Symbol(s) => s.into(),
            Data::Str(s) => format!("{:?}", s),
            Data::Int(i) => format!("{}", i),
            Data::Float(f) => format!("{}", f),
            Data::List(v) => format!("({})", v.iter().map(|x| x.repr()).collect::<Vec<String>>().join(" ")),
            Data::Function(_) => "#fn".into(),
            Data::Nil => "nil".into(),
        }
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repr())
    }
}

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
        interpreter.scopes.push(Interpreter::make_standard_library());

        interpreter
    }

    fn make_standard_library() -> Scope<'a> {
        let mut standard: Scope = HashMap::new();

        standard.insert("hello-world".into(),
        Data::Function(|args| {
            if args.len() == 0 {
                println!("Hello, World!");
                Ok(Data::Nil)
            } else {
                Err("Invalid amount of arguments.".into())
            }}));

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
            Data::List(list) => { // Unquoted list, A.K.A. function call
                if list.len() > 0 {
                    let mut evaluated_list = list.clone();
                    for item in evaluated_list.iter_mut() {
                        *item = match item {
                            Data::List(list) => self.eval(Ok(item.clone())), // Function recursion
                            Data::Symbol(symbol) => match self.scope_lookup(&symbol) { // Try to look up variable
                                Some(thing) => Ok(thing),
                                None => Err(format!(r#"Could not find variable "{}""#, symbol)),
                            },
                            _ => Ok(item.clone()),
                        }?;
                    }

                    match &evaluated_list[0] { // Panic-safe because it's been asserted before that the list length is greater than 0
                        Data::Function(f) => if list.len() == 1 {
                            f(&[])
                        } else {
                            f(&list[1..])
                        },
                        x => Err(format!("Is not a function: {}", x.repr())),
                    }
                } else {
                    Ok(Data::Nil)
                }
            },
            any => Ok(any),
        }
    }

    fn scope_lookup(&self, var_name: &'a str) -> Option<Data> {
        for (_scope_index, scope) in self.scopes.iter().enumerate().rev() {
            if let Some(data) = scope.get/*::<&str>*/(var_name) {
                return Some(data.clone());
            }
        }

        None
    }
}
