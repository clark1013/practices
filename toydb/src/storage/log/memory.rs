use super::LogStore;
use crate::error::{Error, Result};

pub struct MemoryLogStore {
    log: Vec<Vec<u8>>,
    commited: u64,
}

impl LogStore for MemoryLogStore {
    fn append(&mut self, entry: Vec<u8>) -> Result<u64> {
        self.log.push(entry);
        Ok(self.log.len() as u64)
    }

    fn commit(&mut self, index: u64) -> Result<()> {
        if index > self.len() {
            return Err(Error::Internal(format!("index overflow: {}", index)));
        }
        if index < self.commited() {
            return Err(Error::Internal(format!("{} has already commited", index)));
        }
        self.commited = index;
        Ok(())
    }

    fn commited(&self) -> u64 {
        self.commited
    }

    fn len(&self) -> u64 {
        self.log.len() as u64
    }

    fn get(&self, index: u64) -> Result<Option<Vec<u8>>> {
        match index {
            0 => Ok(None),
            idx => Ok(self.log.get(idx as usize - 1).cloned()),
        }
    }
}

impl MemoryLogStore {
    fn new() -> Self {
        MemoryLogStore {
            log: Vec::<Vec<u8>>::new(),
            commited: 0,
        }
    }
}
