use std::fs::File;
use std::io::{self, Read};
use clap::Parser;

const AUTHOR: &str = "This manual page was written by the UwULang team: https://github.com/UwULang";
const VERSION: &str = "0.1.1";
const ABOUT: &str = "uwulang is the best programming language to take over the world!";
const LONG_ABOUT: &str = "uwulang is the best programming language to take over the world! It is a turing-complete language modelled after the measured UwUness of a programming language called BrainFuck";


#[derive(Parser, Debug)]
#[command(author=AUTHOR, version=VERSION, about=ABOUT, long_about=LONG_ABOUT)]
struct Args {
    #[arg(short = 'f', long, help="Provide the .uwu file to be run on")]
    filename: Option<String>,
    #[arg(long)]
    file: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let code =
        if args.filename.is_some() || args.file.is_some() {
            let filename = args.filename.or(args.file).unwrap();
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
