use std::iter::Peekable;
use std::str::Chars;
use crate::error::Result;

// create table t (id int not null primary key, b varchar(15) not null default "");
// insert into t (id, b) values (1, "a"), (2, "b");
// select * from t;

#[derive(PartialEq, Debug)]
pub enum Token {
    Ident(String),
    Number(String),
    String(String),
    Keyword(Keyword),
    Asterisk,
    OpenParen,
    CloseParen,
    Comma,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Create,
    Table,
    Int,
    Not,
    Null,
    Primary,
    Key,
    Varchar,
    Default,
    Insert,
    Into,
    Values,
    From,
}

impl Keyword {
    pub fn from_str(ident: &str) -> Option<Self> {
        Some(match ident.to_uppercase().as_ref() {
            "CREATE" => Self::Create,
            "TABLE" => Self::Table,
            "INT" => Self::Int,
            "NOT" => Self::Not,
            "NULL" => Self::Null,
            "PRIMARY" => Self::Primary,
            "KEY" => Self::Primary,
            "VARCHAR" => Self::Varchar,
            "DEFAULT" => Self::Default,
            "INSERT" => Self::Insert,
            "INTO" => Self::Into,
            "VALUES" => Self::Values,
            "FROM" => Self::From,
            _ => return None,
        })
    }
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
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { iter: input.chars().peekable() }
    }

    fn do_scan(&mut self) -> Result<Option<Token>> {
        self.consume_whitespace();
        match self.iter.peek() {
            Some('"') | Some('\'') => self.scan_string(),
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

    fn next_if<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<char> {
        self.iter.peek().filter(| &c | predicate(*c))?;
        self.iter.next()
    }

    fn scan_ident(&mut self) -> Option<Token> {
        let mut name = self.next_if(| c | c.is_alphabetic())?.to_string();
        while let Some(c) = self.next_if(| c | c.is_alphabetic()) {
            name.push(c)
        };
        Keyword::from_str(&name)
            .map(Token::Keyword)
            .or_else(|| Some(Token::Ident(name.to_lowercase())))
    }

    fn scan_string(&mut self) -> Result<Option<Token>> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Keyword, Token};
    use crate::error::Result;

    #[test]
    fn test_consume_whitespace() {
        let mut lex = Lexer::new("     input");
        lex.consume_whitespace();
        assert_eq!(lex.iter.peek(), Some(&'i'));
    }

    #[test]
    fn test_keyword_and_ident() -> Result<()> {
        let mut lex = Lexer::new("create table t");
        let t1 = lex.do_scan()?;
        assert_eq!(t1, Some(Token::Keyword(Keyword::Create)));
        let t2 = lex.do_scan()?;
        assert_eq!(t2, Some(Token::Keyword(Keyword::Table)));
        let t3 = lex.do_scan()?;
        assert_eq!(t3, Some(Token::Ident("t".to_string())));
        Ok(())
    }
}