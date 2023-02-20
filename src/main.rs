mod tape;
mod token;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

use tape::Tape;
use token::Token;


fn main() -> std::io::Result<()> {
    let code: Vec<char> =
    if env::args().len() == 1 {
        println!("Reading from stdin");
        Vec::new()
    } else {
        let filename = env::args().nth(1).unwrap();
        println!("Reading from file: {}", &filename);
        let mut file = File::open(&filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents.chars().collect()
    };

    assert_eq!(code.iter().filter(|&c| *c == '😒').count(), code.iter().filter(|&c| *c == '😡').count());

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
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                blocks.set(input.chars().next().unwrap() as u8);
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
            None => {
                println!("");
                return Ok(());
            }
        }
        pc += 1;
    }
    println!("");
    Ok(())
}
