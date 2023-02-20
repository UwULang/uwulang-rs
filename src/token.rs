#[derive(Debug, Copy, Clone)]
pub enum Token {
    Increment,
    Decrement,
    Right,
    Left,
    PrintChar,
    ReadChar,
    RandomShort,
    JumpIfZero,
    JumpIfNonZero,
}

impl Token {
    pub fn from_char(c: char) -> Option<Token> {
        match c {
            '👆' => Some(Token::Increment),
            '👇' => Some(Token::Decrement),
            '👉' => Some(Token::Right),
            '👈' => Some(Token::Left),
            '🥺' => Some(Token::PrintChar),
            '😳' => Some(Token::ReadChar),
            '🥴' => Some(Token::RandomShort),
            '😒' => Some(Token::JumpIfZero),
            '😡' => Some(Token::JumpIfNonZero),
            _ => None,
        }
    }
}
