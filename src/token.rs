#[derive(Debug, Copy, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_from_char() {
        assert_eq!(Token::from_char('👆'), Some(Token::Increment));
        assert_eq!(Token::from_char('👇'), Some(Token::Decrement));
        assert_eq!(Token::from_char('👉'), Some(Token::Right));
        assert_eq!(Token::from_char('👈'), Some(Token::Left));
        assert_eq!(Token::from_char('🥺'), Some(Token::PrintChar));
        assert_eq!(Token::from_char('😳'), Some(Token::ReadChar));
        assert_eq!(Token::from_char('🥴'), Some(Token::RandomShort));
        assert_eq!(Token::from_char('😒'), Some(Token::JumpIfZero));
        assert_eq!(Token::from_char('😡'), Some(Token::JumpIfNonZero));
        assert_eq!(Token::from_char('a'), None);
    }
}
