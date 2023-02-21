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


fn run_file(input_filename: &str) -> () {
    let mut input_file = File::open(&input_filename).expect("File not found");
    let mut input_contents = String::new();
    input_file.read_to_string(&mut input_contents).expect("Could not read file");
    let input_contents = input_contents.chars().collect();

    assert!(uwulang::interpret(input_contents).is_ok());
}


run_file_test! {
    test_hello_world: ("tests/cases/hellOwOrld.uwu", "tests/cases/hellOwOrld.out"),
    test_square_10000: ("tests/cases/sqwuare10000.uwu", "tests/cases/sqwuare10000.out"),
}
