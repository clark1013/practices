use crate::storage::kv::{self, encoding};
use super::super::schema::{Catalog, Table};
use crate::error::Result;
use crate::sql::types::{Row, Value};
use bincode;
use serde::{Serialize, Deserialize};

pub struct Engine {
    store: Box<dyn kv::Store>,
}

fn serailize<V: Serialize>(value: &V) -> Result<Vec<u8>> {
    Ok(bincode::serialize(value)?)
}

fn deserialize<'a, V: Deserialize<'a>>(bytes: &'a[u8]) -> Result<V> {
    Ok(bincode::deserialize(bytes)?)
}

impl Catalog for Engine {
    fn create_table(&mut self, table: Table) -> Result<()> {
        // TODO: validate whether table is valid
        self.store.set(&Key::Table(table.name.clone()).encode(), serailize(&table)?)?;
        Ok(())
    }

    fn delete_table(&mut self, table_name: &str) -> Result<()> {
        println!("{}", table_name);
        Ok(())
    }

    fn read_table(&self, table_name: &str) -> Result<Option<Table>> {
        self.store.get(&Key::Table(table_name.to_string()).encode())?.map(| v | deserialize(&v)).transpose()
    }
}

impl super::Trasaction for Engine {
    fn create(&mut self, table_name: &str, row: Row) -> Result<()> {
        let table = self.must_read_table(table_name)?;
        let primary_key = table.get_row_key(&row)?;
        // TODO: validate whether row is valid
        self.store.set(
            &Key::Row(table_name.to_string(), Some(primary_key)).encode(), 
            serailize(&row)?
        )
    }

    fn read(&self, table_name: &str, id: &Value) -> Result<Option<Row>> {
        self.store.get(
            &Key::Row(table_name.to_string(), Some(id.clone())).encode()
        )?
        .map(| v | deserialize(&v))
        .transpose()
    }
}

// TODO: need Cow here
enum Key {
    Table(String),
    Row(String, Option<Value>)
}

impl Key {
    fn encode(self) -> Vec<u8> {
        match self {
            Self::Table(name) => [vec![0x01], encoding::encode_string(name.as_str())].concat(),
            Self::Row(table, None) => [vec![0x02], encoding::encode_string(table.as_str())].concat(),
            Self::Row(table, Some(primary_key)) => {
                [vec![0x02], encoding::encode_string(table.as_str()), encoding::encode_value(&primary_key)].concat()
            }
        }
    }
}