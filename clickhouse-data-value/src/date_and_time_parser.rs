use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/date_and_time.pest"]
pub(crate) struct DateAndTimeParser;
