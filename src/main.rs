use std::fs::File;
use std::io::{self, Read};
use clap::Parser;

const AUTHOR: &str = "This manual page was written by the UwULang team: https://github.com/UwULang";
const VERSION: &str = "0.1.1";
const ABOUT: &str = "uwulang is the best programming language to take over the world!";
const LONG_ABOUT: &str = "uwulang is the best programming language to take over the world! It is a turing-complete language modelled after the measured UwUness of a programming language called BrainFuck";


#[derive(Parser, Debug)]
#[command(author=AUTHOR, version=VERSION, about=ABOUT, long_about=LONG_ABOUT, arg_required_else_help(true))]
struct Args {
    #[arg(short = 'f', long, help="Provide the .uwu file to be run on")]
    filename: Option<String>,
    #[arg()]
    file: Option<String>,
    #[arg(short = 'l', long, help="Load onto tape a csv file of short ints comma separated")]
    preload: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let empty_vec = &mut Vec::new();
    let init_tape_values: &Vec<u8> = if args.preload.is_some() {
        let filename = args.preload.unwrap();
        dbg!("Reading from file: {}", &filename);
        let mut file = File::open(&filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // split by ,
        empty_vec.extend(contents.split(",").map(|s| s.parse::<u8>().expect("CSV file must contain a comma separated list of short ints (0 to 127)")));
        empty_vec
    } else {
        empty_vec
    };
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

    // if the number of 😒 and 😡 are not equal, the program will not run
    assert_eq!(code.iter().filter(|&c| *c == '😒').count(), code.iter().filter(|&c| *c == '😡').count());

    if init_tape_values.len() > 0 {
        dbg!("Preloading tape with values: {:?}", init_tape_values);
        uwulang::interpret_with_init(code, init_tape_values)
    } else {
        uwulang::interpret(code)
    }
}
