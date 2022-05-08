use super::types::{DataType, Value};
use crate::error::Result;

pub trait Catalog {
    fn create_table(&mut self, table: Table) -> Result<()>;
    fn delete_table(&mut self, table_name: &str) -> Result<()>;
    fn read_table(&self, table_name: &str) -> Result<Option<Table>>;
}

pub struct Column {
    name: String,
    data_type: DataType,
    primary_key: bool,
    nullable: bool,
    default: Option<Value>,
    unique: bool,
}

pub struct Table {
    name: String,
    columns: Vec<Column>,
}