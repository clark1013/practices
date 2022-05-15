pub mod kv;
use crate::error::Result;
use super::types::Row;

pub trait Trasaction {
    fn create(&self, table_name: &str, row: Row) -> Result<()>;
    fn read(&self, table_name: &str, row: Row) -> Result<Option<Row>>;
}