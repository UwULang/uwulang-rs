pub mod tape;
pub mod token;

use std::io::{self, Read};
use std::collections::HashMap;

use tape::Tape;
use token::Token;


pub fn interpret(code: Vec<char>) -> std::io::Result<()> {
    let mut temp_bracestack: Vec<usize> = Vec::new();
    let mut jump_table: HashMap<usize, usize> = HashMap::new();
    for (i, c) in code.iter().enumerate() {
        if *c == '😒' {
            temp_bracestack.push(i);
        } else if *c == '😡' {
            let start = temp_bracestack.pop().unwrap();
            jump_table.insert(start, i);
            jump_table.insert(i, start);
        }
    }

    let mut pc: usize = 0;
    let mut blocks = Tape::new();

    while pc < code.len() {
        match Token::from_char(code[pc]) {
            Some(Token::Increment) => {
                blocks.increment();
            },
            Some(Token::Decrement) => {
                blocks.decrement();
            },
            Some(Token::Right) => {
                blocks.move_right();
            },
            Some(Token::Left) => {
                blocks.move_left();
            },
            Some(Token::PrintChar) => {
                print!("{}", blocks);
            },
            Some(Token::ReadChar) => {
                let mut input = vec![0; 1];
                io::stdin().read_exact(&mut input)?;
                blocks.set(input[0]);
            },
            Some(Token::RandomShort) => {
                blocks.set_random();
            },
            Some(Token::JumpIfZero) => {
                if blocks.get() == 0 {
                    pc = jump_table[&pc]
                }
            },
            Some(Token::JumpIfNonZero) => {
                if blocks.get() != 0 {
                    pc = jump_table[&pc]
                }
            },
            None => {}
        }
        pc += 1;
    }
    Ok(())
}
