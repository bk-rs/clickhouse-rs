use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/type_name.pest"]
pub(crate) struct TypeNameParser;
