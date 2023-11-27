use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "arcl.pest"]
pub struct ArclParser;