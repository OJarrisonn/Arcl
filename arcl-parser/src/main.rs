use std::fs;

use parser::ArclParser;
use pest::Parser;

mod parser;

fn main() {
    let source = fs::read_to_string("./code/wrappers.arcl").expect("Failed to read the file");

    let result = ArclParser::parse(parser::Rule::program, &source);
    match result {
        Ok(program) => {
            println!("{program:#?}");
        },
        Err(e) => eprintln!("Failed to parse file. {e}"),
    }
}