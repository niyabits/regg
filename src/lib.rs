use std::{
    fs,
    io::{self, Write},
};

pub struct Regg {}

impl Regg {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_file(path: &str) {
        // TODO: Handle Errors Better
        let bytes = fs::read(path).expect("Error reading file");
        let content = String::from_utf8(bytes).expect("Error parsing file content");

        Regg::run(content);
    }

    pub fn run_prompt() {
        println!("Welcome to REPL of REGG, press CTRL+C to exit.");
        loop {
            print!("> ");
            let mut input = String::new();
            io::stdout().flush().unwrap();

            match io::stdin().read_line(&mut input) {
                Ok(_n) => {
                    Regg::run(String::from(input));
                }
                Err(error) => println!("error: {error}"),
            }
        }
    }

    pub fn run(source: String) -> String {
        println!("{}", source);

        return source;
    }
}
