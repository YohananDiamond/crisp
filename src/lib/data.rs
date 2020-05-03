use crate::lib::interpreter;
use std::fmt;

#[derive(Debug, Clone)]
pub enum DataPre {
    List(Vec<DataPre>),
    Int(String),
    Float(String),
    Str(String),
    Symbol(String),
    Nil,
}

#[derive(Clone)]
pub enum Data {
    List(Vec<Data>),
    Int(i64),
    Float(f64),
    Str(String),
    Symbol(String),
    RustFunction(fn(args: &[Data]) -> interpreter::EvalResult),
    // LispFunction(),
    Nil,
}

impl From<DataPre> for Data {
    fn from(data_pre: DataPre) -> Data {
        match data_pre {
            DataPre::Symbol(s) => Data::Symbol(s),
            DataPre::Str(s) => Data::Str(s),
            DataPre::Int(i) => Data::Int(i.parse::<i64>().unwrap()),
            DataPre::Float(f) => Data::Float(f.parse::<f64>().unwrap()),
            DataPre::List(v) => Data::List(v.iter().map(|pre| Data::from(pre.clone())).collect()),
            DataPre::Nil => Data::Nil,
        }
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repr())
    }
}

impl Data {
    pub fn repr(&self) -> String {
        match self {
            Data::Symbol(s) => s.clone(),
            Data::Str(s) => format!("{:?}", s),
            Data::Int(i) => format!("{}", i),
            Data::Float(f) => format!("{}", f),
            Data::List(v) => format!(
                "({})",
                v.iter().map(Data::repr).collect::<Vec<String>>().join(" ")
            ),
            Data::RustFunction(_) => "#rust/fn".into(),
            Data::Nil => "nil".into(),
        }
    }

    pub fn to_lisp_string(&self) -> String {
        match self {
            Data::Str(s) => s.clone(),
            _ => self.repr(), // The debug repr is able to handle the rest.
        }
    }
}
