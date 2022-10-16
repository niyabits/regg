use clap::Parser;
use regg::Regg;

#[derive(Parser, Debug)]
#[command(name = "Regg")]
#[command(author = "Yash Gupta <yashguptaz@pm.me>")]
#[command(version)]
#[command(about = "Regg is a pet-project tempalting engine written in Rust by @yashguptaz", long_about = None)]
struct Args {
    /// Run Regg on this file
    file: Option<String>,
}

fn main() {
    let mut regg: Regg = Regg::new();
    let args = Args::parse();

    match args.file {
        Some(file) => regg.run_file(&file[..]),
        None => regg.run_prompt(),
    }
}
