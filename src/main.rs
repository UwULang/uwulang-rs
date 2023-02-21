use std::env;
use std::fs::File;
use std::io::Read;

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
    uwulang::interpret(code)
}
