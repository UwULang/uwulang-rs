use std::env;
use std::fs::File;
use std::io::Read;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let code: Vec<char> =
    if args.filename.is_some() {
        let filename = args.filename.unwrap();
        println!("Reading from file: {}", &filename);
        let mut file = File::open(&filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents.chars().collect()
    } else {
        println!("Reading from stdin");
        Vec::new()
    };

    assert_eq!(code.iter().filter(|&c| *c == '😒').count(), code.iter().filter(|&c| *c == '😡').count());
    uwulang::interpret(code)
}
