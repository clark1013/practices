pub mod std_memory;
pub mod encoding;

use std::fmt::Display;
use crate::error::Result;

pub trait Store: Display {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    fn set(&mut self, key: &[u8], value: Vec<u8>) -> Result<()>;
}