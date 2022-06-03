pub mod kv;
use crate::error::Result;
use super::{types::{Row, Value}, schema::Catalog};

pub trait Trasaction: Catalog {
    fn create(&mut self, table_name: &str, row: Row) -> Result<()>;
    fn read(&self, table_name: &str, id: &Value) -> Result<Option<Row>>;
}