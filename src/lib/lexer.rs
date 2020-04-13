#[derive(Debug)]
enum Context {
    Base,
    Symbol,
    Digits,
    FloatDigits,
    String,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    ParenLeft,
    ParenRight,
    Symbol(String),
    Integer(String),
    Float(String),
    Str(String),
}

#[derive(Debug)]
pub enum LexerError {
    UnimplementedToken,
    UnexpectedEOF,
    EmptyContextStack,
    InvalidCharacter(usize, char),
}

#[derive(Debug)]
pub struct Lexer {
    pos: usize,
    contents: Vec<char>,
}

impl From<&String> for Lexer {
    fn from(contents: &String) -> Lexer {
        Lexer {
            pos: 0,
            contents: contents.chars().collect(),
        }
    }
}

impl Lexer {
    pub fn get_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::<Token>::new();
        let mut queue = Vec::<char>::new();
        let mut context_stack = vec![Context::Base];

        loop {
            let mut next_char = true;
            let current_char = self.contents.get(self.pos);

            if let Some(context) = context_stack.last().clone() {
                match context {
                    Context::Base => match current_char {
                        Some(c) => match c {
                            x if x.is_digit(10) => {
                                context_stack.push(Context::Digits);
                                next_char = false;
                            }
                            '"' => {
                                context_stack.push(Context::String);
                            },
                            '(' => tokens.push(Token::ParenLeft),
                            ')' => tokens.push(Token::ParenRight),
                            ' ' => (),
                            '#' | '\'' => return Err(LexerError::UnimplementedToken),
                            _ => {
                                context_stack.push(Context::Symbol);
                                next_char = false;
                            },
                        },
                        None => (),
                    },

                    Context::Digits => match current_char {
                        Some(c) if c.is_digit(10) => queue.push(c.clone()),
                        Some('.') => {
                            queue.push('.');
                            context_stack.pop();
                            context_stack.push(Context::FloatDigits);

                        },
                        Some(' ') | Some(')') | None => {
                            context_stack.pop();
                            tokens.push(Token::Integer(queue.clone().into_iter().collect()));
                            queue.clear();
                            next_char = false;
                        },
                        Some(c) => return Err(LexerError::InvalidCharacter(self.pos, c.clone())),
                    },

                    Context::FloatDigits => match current_char {
                        Some(c) if c.is_digit(10) => queue.push(c.clone()),
                        Some(' ') | Some(')') | None => {
                            context_stack.pop();
                            tokens.push(Token::Float(queue.clone().into_iter().collect()));
                            queue.clear();
                            next_char = false;
                        },
                        Some(c) => return Err(LexerError::InvalidCharacter(self.pos, c.clone())),
                    },

                    Context::String => match current_char {
                        Some('"') => {
                            context_stack.pop();
                            tokens.push(Token::Str(queue.clone().into_iter().collect()));
                            queue.clear();
                        },
                        // TODO: escape sequences;
                        Some(c) => queue.push(c.clone()),
                        None => return Err(LexerError::UnexpectedEOF),
                    },

                    Context::Symbol => match current_char {
                        Some(' ') | Some('#') | Some('\'') | Some('(') | Some(')') | None
                            => {
                                context_stack.pop();
                                tokens.push(Token::Symbol(queue.clone().into_iter().collect()));
                                queue.clear();
                                next_char = false;
                            }
                        Some(c) => queue.push(c.clone()),
                    }
                }
            } else {
                return Err(LexerError::EmptyContextStack);
            }

            if next_char {
                self.pos += 1;
            }
            if let None = current_char {
                break;
            }
        }

        Ok(tokens)
    }
}
