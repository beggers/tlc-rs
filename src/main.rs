use clap::Parser;

use lib::parser::parse_file;
use lib::static_analysis::Analyzer;

#[derive(Parser, Debug)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let file = args.file;
    let ast = parse_file(&file).unwrap();
    let analyzer = Analyzer::new(&ast);
    let _ = analyzer.analyze();
}
