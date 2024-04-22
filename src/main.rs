use clap::Parser;

use lib::parser::parse_file;

#[derive(Parser, Debug)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let file = args.file;
    let ast = parse_file(&file).unwrap();
    println!("{:?}", ast);
}
