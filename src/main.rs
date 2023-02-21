use std::fs::File;
use std::io::{self, Read};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let code =
        if args.filename.is_some() {
            let filename = args.filename.unwrap();
            dbg!("Reading from file: {}", &filename);
            let mut file = File::open(&filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            contents
        } else {
            dbg!("Reading from stdin");
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            let mut buffer = String::new();
            handle.read_to_string(&mut buffer)?;
            buffer
        };
    let code: Vec<char> = code.chars().collect();

    assert_eq!(code.iter().filter(|&c| *c == '😒').count(), code.iter().filter(|&c| *c == '😡').count());
    uwulang::interpret(code)
}
