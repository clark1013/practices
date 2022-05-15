use super::types::{DataType, Value};
use crate::error::Result;
use serde_derive::{Deserialize, Serialize};

pub trait Catalog {
    fn create_table(&mut self, table: Table) -> Result<()>;
    fn delete_table(&mut self, table_name: &str) -> Result<()>;
    fn read_table(&self, table_name: &str) -> Result<Option<Table>>;
}

#[derive(Serialize, Deserialize)]
pub struct Column {
    name: String,
    data_type: DataType,
    primary_key: bool,
    nullable: bool,
    default: Option<Value>,
    unique: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}
