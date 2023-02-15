use super::error::Result;

pub trait KvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()>;

    fn get(&self, key: String) -> Result<Option<String>>;

    fn remove(&mut self, key: String) -> Result<()>;
}

pub use kv::KvStore;
pub use mysled::SledStore;
mod kv;
mod mysled;
