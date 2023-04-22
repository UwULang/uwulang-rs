use uwulang;
use std::fs::File;
use std::io::Read;


macro_rules! run_file_test {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, output) = $value;
                let mut output_file = File::open(&output).expect("File not found");
                let mut output_contents = String::new();
                output_file.read_to_string(&mut output_contents).expect("Could not read file");
                println!("\tExpected output:\n{}", output_contents);
                println!("\tActual output:\n");
                run_file(input);
            }
        )*
    };
}

macro_rules! run_file_test_preload {
    ($($name:ident: $value:expr)*) => {
        $(
            #[test]
            fn $name() {
                let (input, output) = $value;
                let mut output_file = File::open(&output).expect("File not found");
                let mut output_contents = String::new();
                output_file.read_to_string(&mut output_contents).expect("Could not read file");
                println!("\tExpected output:\n{}", output_contents);
                println!("\tActual output:\n");
                run_file_preload(input);
            }
        )*
    };
}


fn run_file(input_filename: &str) -> () {
    let mut input_file = File::open(&input_filename).expect("File not found");
    let mut input_contents = String::new();
    input_file.read_to_string(&mut input_contents).expect("Could not read file");
    let input_contents = input_contents.chars().collect();

    assert!(uwulang::interpret(input_contents).is_ok());
}

fn run_file_preload(input_filename: &str) -> () {
    let mut input_file = File::open(&input_filename).expect("File not found");
    let mut input_contents = String::new();
    input_file.read_to_string(&mut input_contents).expect("Could not read file");
    let input_contents = input_contents.chars().collect();

    assert!(uwulang::interpret_with_init(input_contents, &vec![72,101,108,108,111,32,87,111,114,108,100,33,10]).is_ok());
}


run_file_test! {
    test_hello_world: ("tests/cases/hellOwOrld.uwu", "tests/cases/hellOwOrld.out"),
    test_square_10000: ("tests/cases/sqwuare10000.uwu", "tests/cases/sqwuare10000.out"),
}

run_file_test_preload!{
    test_preload: ("tests/cases/preload.uwu",  "tests/cases/hellOwOrld.out")
}
