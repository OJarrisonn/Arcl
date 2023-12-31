use std::fs;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../utils/grammar.pest"]
struct ArclParser;

fn main() {
    let source = fs::read_to_string("./test/hello_world.arcl").unwrap();


    let program = ArclParser::parse(Rule::program, &source)
        .expect("The syntax is wrong")
        .next()
        .unwrap();

    let _ = dbg!(program);
}
