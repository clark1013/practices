use log::debug;

use crate::error::{KvsError, Result};
use crate::KvsEngine;
use std::path::{Path, PathBuf};

pub struct SledStore {
    db: sled::Db,
}

impl KvsEngine for SledStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let r = self.db.insert(key.as_bytes(), value.as_bytes())?;
        self.db.flush()?;
        debug!("set {} {} {:?}", key, value, r);
        Ok(())
        // match r {
        //     Some(_v) => Ok(()),
        //     None => Err(KvsError::KeyNotFound),
        // }
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        let r = self.db.get(key.as_bytes())?;
        debug!("get {:?} {:?}", key, r);
        match r {
            Some(v) => Ok(Some(String::from_utf8(v.to_vec())?)),
            None => Ok(None),
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let r = self.db.remove(key.as_bytes())?;
        self.db.flush()?;
        match r {
            Some(_v) => Ok(()),
            None => Err(KvsError::KeyNotFound),
        }
    }
}

impl SledStore {
    pub fn open(
        path: impl Into<PathBuf> + std::convert::AsRef<std::path::Path>,
    ) -> Result<SledStore> {
        let db = sled::open(path).unwrap();
        Ok(SledStore { db })
    }
}
