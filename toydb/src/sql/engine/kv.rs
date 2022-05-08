use crate::storage::kv;
use super::super::schema::{Catalog, Table};
use crate::error::Result;

pub struct Engine {
    store: Box<dyn kv::Store>,
}

impl Catalog for Engine {
    fn create_table(&mut self, table: Table) -> Result<()> {
        Ok(())
    }

    fn delete_table(&mut self, table_name: &str) -> Result<()> {
        Ok(())
    }

    fn read_table(&self, table_name: &str) -> Result<Option<Table>> {
        
    }
}

// TODO: need Cow here
enum Key {
    Table(String),
}

impl Key {
    fn encode(self) -> Vec<u8> {
        
    }
}