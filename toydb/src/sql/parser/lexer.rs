use std::iter::Peekable;
use std::str::Chars;
use crate::error::Result;

// create table t (id int not null primary key, b varchar(15) not null default "");
// insert into t (id, b) values (1, "a"), (2, "b");
// select * from t;

pub enum Token {
    Keyword,
    Asterisk,
}

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>
}

impl<'a> Iterator for Lexer<'a> {
    type Item  = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<'a> Lexer<'a> {
    fn scan(&mut self) -> Result<Option<Token>> {
        self.consume_whitespace();
        match self.iter.peek() {
            Some(c) if c.is_alphabetic() => Ok(self.scan_ident()),
            Some(_) => Ok(None),
            None => Ok(None),
        }
    }

    fn consume_whitespace(&mut self) {
        while self.iter.peek().filter(| &c | c.is_whitespace()) != None {
            self.iter.next();
        }
    }

    fn scan_ident(&mut self) -> Option<Token> {
        None
    }
}