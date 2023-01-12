use crate::error::{KvsError, Result};
use crate::KvsEngine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct KvStore {
    path: PathBuf,
    cur_index: u32,
    uncompacted_index: u32,
    mem_table: HashMap<String, u32>,
}

impl KvsEngine for KvStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        if self.cur_index - self.uncompacted_index > 1000 {
            self.compact()?;
        }
        let mut p = self.path.clone();
        p.push(format!("{}.log", self.cur_index));
        self.mem_table.insert(key.clone(), self.cur_index);
        self.cur_index += 1;
        let mut f = File::create(&p)?;
        let command = Command::Set { key, value };
        let j = serde_json::to_string(&command)?;
        f.write_all(j.as_bytes())?;
        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        if let Some(idx) = self.mem_table.get(&key) {
            let file_name = format!("{}.log", *idx);
            let f = File::open(self.path.join(file_name))?;
            let command: Command = serde_json::from_reader(f)?;
            match command {
                Command::Set { key: _, value } => Ok(Some(value)),
                Command::Remove { key: _ } => Err(KvsError::KeyNotFound),
            }
        } else {
            Ok(None)
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        if self.mem_table.get(&key).is_some() {
            self.mem_table.remove(&key);
        } else {
            return Err(KvsError::KeyNotFound);
        }
        let mut p = self.path.clone();
        p.push(format!("{}.log", self.cur_index));
        self.cur_index += 1;
        let mut f = File::create(&p)?;
        let command = Command::Remove { key };
        let j = serde_json::to_string(&command)?;
        f.write_all(j.as_bytes())?;
        Ok(())
    }
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let p = path.into();
        let gen_list = sorted_gen_list(&p)?;
        let mut mem_table: HashMap<String, u32> = HashMap::new();
        for gen in &gen_list {
            let file_name = format!("{}.log", gen);
            let f = File::open(p.join(file_name))?;
            let command: Command = serde_json::from_reader(f)?;
            match command {
                Command::Set { key, value: _ } => mem_table.insert(key, *gen),
                Command::Remove { key } => mem_table.remove(&key),
            };
        }
        let cur_index = gen_list.last().unwrap_or(&0) + 1;
        let store = KvStore {
            path: p,
            uncompacted_index: 1,
            cur_index,
            mem_table,
        };
        Ok(store)
    }

    fn compact(&mut self) -> Result<()> {
        for i in self.uncompacted_index..self.cur_index {
            let file_name = format!("{}.log", i);
            let f = File::open(self.path.join(&file_name))?;
            let command: Command = serde_json::from_reader(f)?;
            let mut should_delete = true;
            match command {
                Command::Set { key, value: _ } => {
                    if let Some(idx) = self.mem_table.get(&key) {
                        if *idx == i {
                            should_delete = false;
                        }
                    }
                }
                _ => {}
            }
            if should_delete {
                fs::remove_file(self.path.join(&file_name))?;
            }
        }
        self.uncompacted_index = self.cur_index;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

fn sorted_gen_list(path: &Path) -> Result<Vec<u32>> {
    let mut indexes: Vec<u32> = fs::read_dir(path)?
        .flat_map(|res| -> Result<_> { Ok(res?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u32>)
        })
        .flatten()
        .collect();
    indexes.sort();
    Ok(indexes)
}
