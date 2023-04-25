use crate::{
    binder::Binder,
    parser::{parse, ParserError},
};
use thiserror::Error;

pub struct Database {}

impl Database {
    pub fn new() -> Self {
        Database {}
    }

    pub fn run(&self, sql: &str) -> Result<(), Error> {
        let stmts = parse(sql)?;
        for stmt in stmts {
            let binder = Binder::new();
            binder.bind(&stmt);
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("parse error: {0}")]
    Parse(#[from] ParserError),
}
