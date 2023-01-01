use std::fs;
use clap::builder::Str;
use bbbasic;
use clap::Parser as Clapper;

#[derive(Clapper, Debug)]
#[clap(author = "Stephan Smola", version = "0.1", about = "Basic Interpreter inspired by BBC Basic", long_about = None)]
struct Args {
    /// Filename of a .bbb-File
    filename: String
}



fn main() {
    let args = Args::parse();

    match fs::read_to_string(&args.filename) {
        Ok(code) => {
            bbbasic::execute(code.as_str());
        }
        Err(e) => {
            eprintln!("File not found: {}", args.filename);
        }
    }

}