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
    let args = Args::parse();

    match args.file {
        Some(file) => Regg::run_file(&file[..]),
        None => Regg::run_prompt(),
    }
}
