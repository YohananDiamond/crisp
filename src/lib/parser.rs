use crate::lib::lexer::Token;

pub type ParserResult = Result<Vec<Expression>, ParserError>;

#[derive(Debug)]
#[derive(Clone)]
pub enum Expression {
    Symbol(String),
    Str(String),
    Int(i64),
    Float(f64),
    List(Vec<Expression>),
}

#[derive(Debug)]
pub enum ParserError {
    UnmatchedParenLeft,
    UnmatchedParenRight,
}

pub fn parse_tokens(tokens: &Vec<Token>) -> Result<Vec<Expression>, ParserError> {
    let mut position: usize = 0;
    let mut queue_stack: Vec<Vec<Expression>> = Vec::new(); // If empty, the parser is on the "root"; if else, then it is on a list
    let mut expressions: Vec<Expression> = Vec::new();

    loop {
        let mut next_char = true;
        let current_token = tokens.get(position).clone();

        if let Some(token) = current_token {
            match token {
                Token::ParenLeft => queue_stack.push(Vec::new()),
                Token::ParenRight => match queue_stack.pop() {
                    Some(q) => match queue_stack.last_mut() {
                        Some(m) => m.push(Expression::List(q)),
                        None => expressions.push(Expression::List(q)),
                    },
                    None => return Err(ParserError::UnmatchedParenRight),
                },
                other_token => match queue_stack.last_mut() {
                    Some(q) => match other_token {
                        Token::ParenLeft | Token::ParenRight => (), // Already covered before
                        Token::Symbol(s) => q.push(Expression::Symbol(s.to_string())),
                        Token::Str(s) => q.push(Expression::Str(s.to_string())),
                        // Calls to .unwrap() here are safe because the contents of these tokens
                        // were already analyzed in the lexer.
                        Token::Integer(i) => q.push(Expression::Int(i.parse::<i64>().unwrap())),
                        Token::Float(f) => q.push(Expression::Float(f.parse::<f64>().unwrap())),
                    },
                    None => match other_token {
                        Token::ParenLeft | Token::ParenRight => (), // Already covered before
                        Token::Symbol(s) => expressions.push(Expression::Symbol(s.to_string())),
                        Token::Str(s) => expressions.push(Expression::Str(s.to_string())),
                        // Calls to .unwrap() here are safe because the contents of these tokens
                        // were already analyzed in the lexer.
                        Token::Integer(i) => expressions.push(Expression::Int(i.parse::<i64>().unwrap())),
                        Token::Float(f) => expressions.push(Expression::Float(f.parse::<f64>().unwrap())),
                    },
                }
            }
        }

        if next_char {
            if let None = current_token {
                if queue_stack.len() != 0 {
                    return Err(ParserError::UnmatchedParenLeft);
                }
                break;
            }
            position += 1;
        }
    }

    Ok(expressions)
}
