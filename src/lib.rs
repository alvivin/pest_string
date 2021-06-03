pub mod expr;
pub mod parser;
pub mod scanner;

pub trait StringParser {
    fn parse_string(input: String) -> Result<Vec<expr::Stmt>, parser::Error>;
}
