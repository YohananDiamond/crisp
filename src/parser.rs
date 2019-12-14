pub enum Operator {
    Add,
}

pub enum Token {
    Integer(i32),
    Operator(Operator),
}

/**
 * Parses a string and returns a vector with tokens.
 */
pub fn parse_string(string: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut queue: Vec<char> = Vec::new();

    for (i, c) in string.chars().enumerate() {
        println!("Char {:?} at index {}", c, i);
        queue.push(c);
    }

    println!("{:?}", queue);
    let a: String = queue.into_iter().collect();
    println!("{:?}", a);

    tokens
}
