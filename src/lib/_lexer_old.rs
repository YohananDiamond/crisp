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
    chars_consumed: usize, // NOTE: position in the code is chars_consumed - 1
    contents: Vec<char>,
    queue: Vec<char>,
    context_stack: Vec<Context>,
}

impl<'a> From<&'a str> for Lexer {
    fn from(code: &'a str) -> Lexer {
        Lexer {
            chars_consumed: 0,
            contents: code.chars().collect(),
            queue: Vec::new(),
            context_stack: vec![Context::Base],
        }
    }
}

impl Lexer {
    pub fn queue_to_string(queue: &mut Vec<char>) -> String {
        let mut string = String::new();
        while queue.len() != 0 {
            string.push(queue.remove(0));
        }
        string
    }

    /// Get a single token, consuming part of the contents.
    pub fn get_token(&mut self) -> Result<Option<Token>, LexerError> {
        let token = loop {
            let mut continue_consuming = true;
            let current_char = self.contents.get(0);
            if let Some(c) = current_char {
                self.chars_consumed += 1;
            }

            if let Some(context) = context_stack.last() {
                match context {
                    &Context::Base => if let Some(c) = current_char {
                        x if x.is_digit(10) => {
                            self.context_stack.push(Context::Digits);
                            continue_consuming = false;
                        }
                        '"' => self.context_stack.push(Context::String),
                        '(' => break Some(Token::ParenLeft),
                        ')' => break Some(Token::ParenRight),
                        ' ' => {},
                        '#' | '\'' => return Err(LexerError::UnimplementedToken),
                        _ => {
                            self.context_stack.push(Context::Symbol);
                            continue_consuming = false;
                        },
                    } else {
                        break None;
                    },

                    &Context::Digits => match current_char {
                        Some(c) if c.is_digit(10) => queue.push(c.clone()),
                        Some('.') => {
                            queue.push('.');
                            self.context_stack.pop();
                            self.context_stack.push(Context::FloatDigits);
                        },
                        Some(' ') | Some(')') | None => {
                            self.context_stack.pop();
                            break Some(Token::Integer(Lexer::queue_to_string(&mut queue)));
                        },
                        Some(c) => return Err(LexerError::InvalidCharacter(self.pos, c.clone())),
                    },

                    &Context::FloatDigits => match current_char {
                        Some(c) if c.is_digit(10) => queue.push(c.clone()),
                        Some(' ') | Some(')') | None => {
                            self.context_stack.pop();
                            break Some(Token::Float(Lexer::queue_to_string(&mut queue)));
                            continue_consuming = false;
                        },
                        Some(c) => return Err(LexerError::InvalidCharacter(self.pos, c.clone())),
                    },

                    &Context::String => match current_char {
                        Some('"') => {
                            self.context_stack.pop();
                            break Some(Token::Str(Lexer::queue_to_string(&mut queue)));
                        },
                        // TODO: escape sequences;
                        Some(c) => queue.push(c.clone()),
                        None => return Err(LexerError::UnexpectedEOF),
                    },

                    &Context::Symbol => match current_char {
                        Some(' ') | Some('#') | Some('\'') | Some('(') | Some(')') | None
                            => {
                                self.context_stack.pop();
                                tokens.push(Token::Symbol(Lexer::queue_to_string(&mut queue)));
                                queue.clear();
                                continue_consuming = false;
                            }
                        Some(c) => queue.push(c.clone()),
                    }
                }
            } else {
                return Err(LexerError::EmptyContextStack);
            }

            if continue_consuming {
                if let Some(c) = current_char {
                    self.remove(0);
                }
            }
            if let None = current_char {
                break;
            }
        }

        Ok(tokens)
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token>, LexerError> {}
}
