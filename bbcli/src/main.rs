use std::fs;
use std::time::Instant;
use clap::builder::Str;
use bbbasic;
use clap::Parser as Clapper;

#[derive(Clapper, Debug)]
#[clap(author = "Stephan Smola", version = "0.1", about = "Basic Interpreter inspired by BBC Basic", long_about = None)]
struct Args {

    /// Run as benchmark.
    #[clap(short = 'b', long, action)]
    benchmark: bool,

    /// Benchmark repetitions
    #[clap(short = 'c', long, default_value = "20")]
    count: u32,



    /// Filename of a .bbb-File
    filename: String

}


fn benchmark(filename: &String, count: u32) {
    match fs::read_to_string(filename) {
        Ok(code) => {
            let t1 = Instant::now();
            for _ in 0..count {
                bbbasic::execute(code.as_str());
            }
            let elapsed = t1.elapsed();

            println!("Running {} {} times:", filename, count);
            println!("Total time:\t\t{:.2?}", elapsed);
            println!("Avg. single exec:\t{:.2?}", elapsed / count);

        }
        Err(e) => {
            eprintln!("File not found: {}", filename);
        }
    }
}

fn run(filename: &String) {
    match fs::read_to_string(filename) {
        Ok(code) => {
            bbbasic::execute(code.as_str());
        }
        Err(e) => {
            eprintln!("File not found: {}", filename);
        }
    }
}

fn main() {
    let args = Args::parse();

    if args.benchmark {
        benchmark(&args.filename, args.count)
    } else {
        run(&args.filename);
    }
}