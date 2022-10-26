use std::{
    fs,
    io::{self, Write},
};

use crate::scanner::Scanner;

pub mod scanner;
pub mod token;
pub mod token_type;

pub struct Regg {
    had_error: bool,
}

impl Regg {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) {
        // TODO: Handle Errors Better
        let content = fs::read_to_string(path).expect("Error reading file");

        self.run(&content);

        if self.had_error {
            std::process::exit(65)
        }
    }

    pub fn run_prompt(&mut self) {
        print!("Welcome to REPL of REGG, press CTRL+C to exit.\n");
        loop {
            print!("> ");
            let mut input = String::new();
            io::stdout().flush().unwrap();

            match io::stdin().read_line(&mut input) {
                Ok(_n) => {
                    self.run(&input);
                    self.had_error = false;
                }
                Err(error) => println!("error: {error}"),
            }
        }
    }

    pub fn run<'a>(&mut self, source: &'a str) -> &'a str {
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        tokens.iter().for_each(|token| println!("{:?}", token));

        return source;
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, place: &str, message: &str) {
        // TODO: improve error handling
        // https://github.com/zkat/miette
        // http://www.unicode.org/charts/PDF/U2500.pdf
        //    ╭
        // 15 │ <h1> {, heading.toUpperCase() } </h1>
        //    .       ┬
        //    .       │
        //    .       └─ unexpected character `,`

        println!("[line {}] Error{}: {}", line, place, message);

        self.had_error = true;
    }
}
