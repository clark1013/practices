use super::types::{DataType, Value, Row};
use crate::error::{Result, Error};
use serde_derive::{Deserialize, Serialize};

pub trait Catalog {
    fn create_table(&mut self, table: Table) -> Result<()>;
    fn delete_table(&mut self, table_name: &str) -> Result<()>;
    fn read_table(&self, table_name: &str) -> Result<Option<Table>>;
    fn must_read_table(&self, table_name: &str) -> Result<Table> {
        self.read_table(table_name)?.ok_or_else(|| Error::Value(format!("table {} does not exsist", table_name)))
    }
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

impl Table {
    pub fn get_row_key(&self, row: &Row) -> Result<Value> {
        row.get(
            self.columns
            .iter()
            .position(| c | c.primary_key)
            .ok_or_else(|| Error::Value("Primary key not found".into()))?
        )
        .cloned()
        .ok_or_else(|| Error::Value("Primary key column not found for row".into()))
    }
}