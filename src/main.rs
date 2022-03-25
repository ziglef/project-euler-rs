use clap::Parser;

mod problems;
use problems::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 0)]
    problem_no: u32,
}

fn main() {
    let args = Args::parse();

    match args.problem_no {
        1 => println!("{:?}", p1::p1(1000)),
        0 => println!("Please choose a valid problem number. If you don't know how, check --help"),
        _ => println!("Problem not yet implemented!")
    }
}
