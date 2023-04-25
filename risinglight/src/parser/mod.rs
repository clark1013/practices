pub use sqlparser::ast::*;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
pub use sqlparser::parser::ParserError;

pub fn parse(sql: &str) -> Result<Vec<Statement>, ParserError> {
    let dialect = GenericDialect {};
    Parser::parse_sql(&dialect, sql)
}
