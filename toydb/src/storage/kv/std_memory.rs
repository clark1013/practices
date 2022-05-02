use std::collections::BTreeMap;
use crate::error::Result;
use super::Store;
use std::fmt::Display;

struct StdMemory {
    data: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl StdMemory {
    fn new() -> Self {
        Self {data: BTreeMap::new()}
    }
}

impl Display for StdMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "stdmemory")
    }
}

impl Store for StdMemory {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.data.get(key).cloned())
    }

    fn set(&mut self, key: &[u8], value: Vec<u8>) -> Result<()> {
        self.data.insert(key.to_vec(), value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::kv::Store;

    use super::StdMemory;

    #[test]
    fn test_get() -> crate::error::Result<()> {
        let mut s = StdMemory::new();
        s.set(b"a", vec![0x01])?;
        assert_eq!(s.get(b"a")?, Some(vec![0x01]));
        assert_eq!(s.get(b"b")?, None);
        Ok(())
    }

    #[test]
    fn test_set() -> crate::error::Result<()> {
        let mut s = StdMemory::new();
        s.set(b"a", vec![0x01])?;
        assert_eq!(s.get(b"a")?, Some(vec![0x01]));
        s.set(b"a", vec![0x02])?;
        assert_eq!(s.get(b"a")?, Some(vec![0x02]));
        Ok(())
    }
}