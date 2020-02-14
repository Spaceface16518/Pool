use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "behavior.pest"]
pub struct BehaviorParser;