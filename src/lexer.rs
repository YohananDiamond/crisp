#[derive(Debug)]
pub enum Context {
    Surface,
    Digits,
    String,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    EOL,
    Integer(String),
    String(String),
    Operator(OpCode),
    OpenParen,
    CloseParen,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum OpCode {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct Lexer {
    content: Vec<char>,
    position: usize,
}

// Static Methods
impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            content: Vec::new(),
            position: 0,
        }
    }
    pub fn with_content(content: &str) -> Lexer {
        Lexer {
            content: content.chars().collect(),
            position: 0,
        }
    }
}

// Class Methods
impl Lexer {

    /// Gets the current char
    pub fn current_char(&self) -> Option<&char> {
        self.content.get(self.position)
    }

    /// Adds 1 to the position and returns it.
    pub fn next_char(&mut self) -> Option<&char> {
        self.position += 1 as usize;
        self.current_char()
    }

    /// Gets all tokens since the current position
    pub fn get_all_next_tokens(&mut self) -> Vec<Token> {
        let mut vec: Vec<Token> = Vec::new();
        loop {
            let token = self.get_next_token();
            let gonna_break = token == Token::EOL;
            vec.push(token);
            if gonna_break {
                break;
            }
        }
        vec
    }

    /// Gets the next token
    pub fn get_next_token(&mut self) -> Token {
        let mut context = vec![Context::Surface];
        let mut queue: Vec<char> = Vec::new();

        loop {
            let c = self.current_char();
            let mut final_token: Option<Token> = None; // Should this be local to this loop?
            let mut jump_next_char = true;

            match context.last().clone() {
                Some(Context::Surface) => match c {
                    Some(t) => match t {
                        x if x.is_digit(10) => {
                            context.push(Context::Digits);
                            jump_next_char = false;
                        }
                        '+' => final_token = Some(Token::Operator(OpCode::Add)),
                        '-' => final_token = Some(Token::Operator(OpCode::Subtract)),
                        '*' => final_token = Some(Token::Operator(OpCode::Multiply)),
                        '/' => final_token = Some(Token::Operator(OpCode::Divide)),
                        '(' => final_token = Some(Token::OpenParen),
                        ')' => final_token = Some(Token::CloseParen),
                        '"' => context.push(Context::String),
                        ' ' => {},
                        _ => println!("Invalid char: ({}, {:?})", self.position, c),
                    }

                    None => final_token = Some(Token::EOL),
                }

                Some(Context::Digits) => match c {
                    Some(x) if x.is_digit(10) => queue.push(*x),
                    _ => {
                        jump_next_char = false;
                        final_token = Some(Token::Integer(queue.clone().into_iter().collect()));
                    }
                }

                Some(Context::String) => match c {
                    Some('"') => final_token = Some(Token::String(queue.clone().into_iter().collect())),
                    Some(x) => queue.push(*x),
                    None => panic!("EOF before end of string"),
                }

                None => println!("The context stack is empty (how?)"),
            }

            // Jump to the next char, according to the `jump_next_char` variable.
            if jump_next_char {
                self.next_char();
            }

            // Return the token if it actually exists
            if let Some(t) = final_token {
                return t
            }
        }
    }

}
