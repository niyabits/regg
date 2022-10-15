use std::{
    fs,
    io::{self, Write},
};

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

    pub fn run<'a>(&mut self, source: &'a String) -> &'a String {
        println!("{}", source);

        return source;
    }

    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: u32, place: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, place, message);

        self.had_error = true;
    }
}
