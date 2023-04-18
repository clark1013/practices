pub mod memory;

pub use memory::MemoryLogStore;

use crate::error::Result;

pub trait LogStore {
    fn append(&mut self, entry: Vec<u8>) -> Result<u64>;
    fn commit(&mut self, index: u64) -> Result<()>;
    fn commited(&self) -> u64;
    fn get(&self, index: u64) -> Result<Option<Vec<u8>>>;
    fn len(&self) -> u64;
}
